#!/bin/bash

/opt/devkitpro/tools/bin/grit page_pic.png -gu16 -gb -gB8 -ab16 -pn16 -ftbin -fh! -ofront
/opt/devkitpro/tools/bin/grit page_pic.png -gu16 -gb -gB8 -ab32 -at16 -pn16 -p! -ftbin -fh! -oback
/opt/devkitpro/tools/bin/grit page_pic.png -gu16 -gb -gB8 -ab16 -pn16 -ofront2
