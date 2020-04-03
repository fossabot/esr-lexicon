set -e

choco install -r --no-progress -y msys2 make
PATH=$PATH:/c/tools/msys64/usr/bin/

# Calling pacman via a powershell script
powershell -executionpolicy bypass "pacman -Syu --noconfirm autoconf libtool automake make autoconf-archive pkg-config"

# Fix the environment for autotools to actually work
ln -s /c/tools/msys64/usr/share/autoconf* /usr/share/
ln -s /c/tools/msys64/usr/share/automake* /usr/share/
ln -s /c/tools/msys64/usr/share/aclocal* /usr/share/
ln -s /c/tools/msys64/usr/share/libtool* /usr/share/
ln -s /c/tools/msys64/usr/share/pkgconfig /usr/share/
ln -s /c/tools/msys64/usr/bin/autom4te /usr/bin/
ln -s /c/tools/msys64/usr/bin/autoconf /usr/bin/
ln -s /c/tools/msys64/usr/bin/autoheader /usr/bin/
ln -s /c/tools/msys64/usr/bin/m4 /usr/bin/

PATH=$PATH:/c/tools/msys64/usr/bin/
export CONFIG_SHELL=/usr/bin/bash.exe

autoreconf -vfi

./configure 
# HACK: Ensure that msys64 comes first, as otherwise libtool won't work
PATH=/c/tools/msys64/usr/bin/:$PATH
