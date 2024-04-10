#!/usr/bin/env nu

print "Minifying css.."
pnpx tailwindcss -i ./input.css -o ./assets/tailwind.min.css --minify

print "Building the app.."
dx build --release --features bundle

let release = ('release\' | path exists)
if $release {
    print "Cleaning rlease folder.."
    rm -r release\
}

print "Copying data.."
mkdir release\
# Copy only minified data
cp assets\*.min.css release\
# Copy icons
cp assets\*.ico release\
cp dist\html-to-rsx-ui.exe release\

print "Bundling the app.."
cd release\
7z a html-to-rsx-ui.7z *
cd ..
