pkgver=`cargo pkgid | cut -d "@" -f2`

cargo publish

cargo aur

gh release create v$pkgver hyprland-relative-workspace-$pkgver-x86_64.tar.gz --generate-notes -t $pkgver
mv hyprland-relative-workspace-$pkgver-x86_64.tar.gz releases/

cp PKGBUILD hyprland-relative-workspace-bin
cd hyprland-relative-workspace-bin
makepkg --printsrcinfo > .SRCINFO
git add .SRCINFO PKGBUILD
git commit -m "`cargo pkgid | cut -d "#" -f2`"
git push origin master

cd ..
git add .
git commit -m "Released version: $pkgver"
git pull
git push