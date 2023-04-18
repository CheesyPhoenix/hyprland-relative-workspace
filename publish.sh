cargo publish

cargo aur
rm hyprland-relative-workspace-*-x86_64.tar.gz
cp PKGBUILD hyprland-relative-workspace-bin
cd hyprland-relative-workspace-bin
makepkg --printsrcinfo > .SRCINFO
git add .SRCINFO PKGBUILD
git commit -m "`cargo pkgid | cut -d "#" -f2`"
git push origin master