#!/bin/bash

/opt/devkitpro/tools/bin/grit page_pic.png -gu16 -gb -gB8 -ab16 -pn16 -ftb -fh! -ofront
/opt/devkitpro/tools/bin/grit page_pic.png -gu16 -gb -gB8 -ab32 -at16 -p! -ftb -fh! -oback

/opt/devkitpro/tools/bin/grit modes.png -gu16 -gb -gB16 -ftb -p -pn16 -fh!
/opt/devkitpro/tools/bin/grit modes.png -g! -gu16 -gb -gB8 -ftb -p -pn16 -fh!

/opt/devkitpro/tools/bin/grit gba.png -gu32 -gb -gB8 -p -pu32 -pn16 -ftb -fh! -ogba_pic
