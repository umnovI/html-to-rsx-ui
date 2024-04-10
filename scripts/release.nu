#!/usr/bin/env nu

dx build --release --features bundle

let release = ('release\' | path exists)
if $release {
    rm -r release\
}

mkdir release\
cp assets\* release\
cp dist\html-to-rsx-ui.exe release\

cd release\
7z a html-to-rsx-ui.7z *
cd ..
