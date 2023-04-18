cargo publish

makepkg -g
rm hyprland-relative-workspace-*.tar.gz
cp PKGBUILD hyprland-relative-workspace-bin
cd hyprland-relative-workspace-bin
sed -i "s/pkgver=[0-9]\.[0-9]\.[0-9]/pkgver=`cargo pkgid | cut -d "@" -f2`/" PKGBUILD
makepkg --printsrcinfo > .SRCINFO
git add .SRCINFO PKGBUILD
git commit -m "`cargo pkgid | cut -d "#" -f2`"
git push origin master