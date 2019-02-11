#![feature(asm)]
#![feature(slice_patterns)]
#![no_std]
#![no_main]

extern crate panic_halt;

use gba::{data,  mmio::{self, Dispcnt}, input::{self, Keyinput},
          obj::{self, *}, obj_aff::{self, Fixed, ObjAffine}, util, video};
use gba_boot::entry;
use core::{mem, slice};

#[derive(PartialEq, Eq)]
enum AffineState {
    Null,
    Rotate,
    ScaleX,
    ScaleY,
    ShearX,
    ShearY
}

fn init_metr() {
    let metr_bitmap = include_bytes!("res/metr2.img.bin");
    let metr_pal = include_bytes!("res/metr2.pal.bin");

    unsafe {
        data::load_obj_bitmap(0, metr_bitmap);
        data::load_obj_palette(0, metr_pal);
    }
}

fn get_affine_state(keys: u16) -> AffineState {
    if input::is_down(keys, Keyinput::BUTTON_L::SET + Keyinput::BUTTON_R::SET) {
        AffineState::Rotate
    }
    else if input::is_down(keys, Keyinput::BUTTON_A::SET) {
        AffineState::ScaleX
    }
    else if input::is_down(keys, Keyinput::BUTTON_B::SET) {
        AffineState::ScaleY
    }
    else if input::is_down(keys, Keyinput::PAD_LEFT::SET + Keyinput::PAD_RIGHT::SET) {
        AffineState::ShearX
    }
    else if input::is_down(keys, Keyinput::PAD_UP::SET + Keyinput::PAD_DOWN::SET) {
        AffineState::ShearY
    }
    else {
        AffineState::Null
    }
}

fn get_affine_new(aff: &mut ObjAffine, state: AffineState, curr_keys: u16, aff_value: &mut Fixed) {
    let mut diff = match state {
        AffineState::Null   => 0,
        AffineState::Rotate => 128,
        AffineState::ScaleX => 4,
        AffineState::ScaleY => 4,
        AffineState::ShearX => 4,
        AffineState::ShearY => 4,
    };

    let key = match state {
        AffineState::Null   => Keyinput::NONE::CLEAR,
        AffineState::Rotate => Keyinput::BUTTON_L::SET,
        AffineState::ScaleX => Keyinput::SELECT::SET,
        AffineState::ScaleY => Keyinput::SELECT::SET,
        AffineState::ShearX => Keyinput::PAD_RIGHT::SET,
        AffineState::ShearY => Keyinput::PAD_UP::SET,
    };

    if !input::is_down(curr_keys, key) {
        diff = -diff;
    }

    *aff_value += Fixed::from_bits(diff);

    match state {
        AffineState::Null   => aff.identity(),
        AffineState::Rotate => aff.rotate(aff_value.to_bits() as u16),
        AffineState::ScaleX => aff.scale_inv(Fixed::from_int(1).wrapping_sub(*aff_value), Fixed::from_int(1)),
        AffineState::ScaleY => aff.scale_inv(Fixed::from_int(1), Fixed::from_int(1).wrapping_sub(*aff_value)),
        AffineState::ShearX => aff.shear_x(*aff_value),
        AffineState::ShearY => aff.shear_y(*aff_value),
    }
}

#[entry]
fn main() -> ! {
    init_metr();

    let mmio = mmio::get_mut();
    mmio.dispcnt.write(Dispcnt::SCR_MODE::Bg0
                     + Dispcnt::SCR_MODE::Obj
                     + Dispcnt::OBJ_DIM::OneDim);

    let mut obj_buffer: [ObjAttr; 128] = unsafe { mem::uninitialized() };
    obj::init_slice(&mut obj_buffer);

    {
        let &mut [ref mut metr, ref mut shadow, _..] = &mut obj_buffer;

        metr.attr0.write(Attr0::OBJ_SHAPE::Square + Attr0::AFFINE::SET);
        metr.attr1.write(Attr1::OBJ_SIZE::Square64 + Attr1::AFFINE_ID.val(0));
        metr.attr2.write(Attr2::PALBANK.val(0));
        metr.set_pos(96, 32);

        shadow.attr0.write(Attr0::OBJ_SHAPE::Square + Attr0::AFFINE::SET);
        shadow.attr1.write(Attr1::OBJ_SIZE::Square64 + Attr1::AFFINE_ID.val(31));
        shadow.attr2.write(Attr2::PALBANK.val(1));
        shadow.set_pos(96, 32);
    }

    obj::copy_slice(&obj_buffer);

    // The affine data is interleaved with the object attribute data, so it becomes necessary to
    // use unsafe Rust in order to use modify both kinds at once.
    let obj_aff_buffer = obj_buffer.as_mut_ptr() as *mut ObjAffine;
    unsafe {
        let mut obj_aff_slice = slice::from_raw_parts_mut(obj_aff_buffer, 32);
        obj_aff::init_slice(&mut obj_aff_slice);
        obj_aff::copy_slice(&obj_aff_slice);
    }

    let aff_curr = obj_aff_buffer;
    let aff_base;
    let aff_new;

    unsafe {
        aff_base = obj_aff_buffer.add(1);
        aff_new = obj_aff_buffer.add(2);
    }

    let mut x = 96;
    let mut y = 32;
    let mut prev_keys = 0;
    let mut aff_state = AffineState::Null;
    let mut new_state;
    let mut aff_value = Fixed::from_int(0);

    loop {
        let curr_keys = input::poll();

        if input::is_down(curr_keys, Keyinput::SELECT::SET)
            && input::is_down(curr_keys, Keyinput::PAD::Any)
        {
            x += 2 * input::tri_pad_horz();
            y += 2 * input::tri_pad_vert();

            {
                let &mut [ref mut metr, ref mut shadow, _..] = &mut obj_buffer;
                metr.set_pos(x as u16, y as u16);
                shadow.set_pos(x as u16, y as u16);
            }
            new_state = AffineState::Null;
        }
        else
        {
            new_state = get_affine_state(curr_keys);
        }

        if new_state != AffineState::Null {
            if new_state == aff_state {
                unsafe {
                    get_affine_new(aff_new.as_mut().unwrap(), aff_state, curr_keys, &mut aff_value);
                    aff_curr.as_mut().unwrap().copy_from(aff_base.as_ref().unwrap());
                    aff_curr.as_mut().unwrap().postmul(aff_new.as_ref().unwrap());
                }
            }
            else
            {
                unsafe {
                    aff_base.as_mut().unwrap().copy_from(aff_curr.as_ref().unwrap());
                    aff_curr.as_mut().unwrap().copy_from(aff_base.as_ref().unwrap());
                    aff_value = Fixed::from_int(0);
                }
            }
            aff_state = new_state;
        }

        if input::was_hit_now(curr_keys, prev_keys, Keyinput::START::SET) {
            if input::is_down(curr_keys, Keyinput::SELECT::SET) {
                unsafe {
                    aff_curr.as_mut().unwrap().identity();
                    aff_base.as_mut().unwrap().identity();
                    aff_new.as_mut().unwrap().identity();
                }
                aff_value = Fixed::from_int(0);
            }
            else {
                {
                    let &mut [ref mut metr, ref mut shadow, _..] = &mut obj_buffer;
                    util::flip_flag(&metr.attr0, Attr0::SCALING);
                    util::flip_flag(&shadow.attr0, Attr0::SCALING);
                }
            }
        }

        video::vsync();

        obj::copy_slice(&obj_buffer);
        unsafe {
            obj_aff::copy_slice(slice::from_raw_parts(obj_aff_buffer, 32));
        }

        prev_keys = curr_keys;
    }
}
