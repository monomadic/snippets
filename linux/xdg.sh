
# defaults
xdg-settings get default-web-browser
BROWSER=firefox xdg-settings get default-web-browser

# mime types
xdg-mime query default text/html

# the default 'show in folder' for chromium is using xdg-open
# to verify what that default is,
xdg-mime query default inode/directory


# note linopen is a simpler variant of xdg-open https://github.com/Cloudef/PKGBUILDS/tree/master/linopen
