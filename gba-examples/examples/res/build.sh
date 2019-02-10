#!/bin/bash

/opt/devkitpro/tools/bin/grit page_pic.png -gu16 -gb -gB8 -ab16 -pn16 -ftb -fh! -ofront
/opt/devkitpro/tools/bin/grit page_pic.png -gu16 -gb -gB8 -ab32 -at16 -p! -ftb -fh! -oback

/opt/devkitpro/tools/bin/grit modes.png -gu16 -gb -gB16 -ftb -p -pn16 -fh!
/opt/devkitpro/tools/bin/grit modes.png -g! -gu16 -gb -gB8 -ftb -p -pn16 -fh!

/opt/devkitpro/tools/bin/grit gba.png -gu32 -gb -gB8 -p -pu32 -pn16 -ftb -fh! -ogba_pic

/opt/devkitpro/tools/bin/grit metr.png -gu32 -gt -gB4 -ar64 -p -pu32 -pn32 -ftb -fh!

/opt/devkitpro/tools/bin/grit brin-full.png -gu16 -gt -gB4 -p -pu16 -pn256 -m -mu16 -mLs -mRtpf -ftb -fh! -obrin

/opt/devkitpro/tools/bin/grit cbb_ids.png -gu32 -gt -gB8 -p! -ar72 -at8 -ab16 -ftb -fh! -oids8
/opt/devkitpro/tools/bin/grit cbb_ids.png -gu32 -gt -gB4 -p -pu32 -pn16 -ar40 -ab8 -ftb -fh! -oids4
