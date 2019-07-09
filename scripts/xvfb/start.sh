#! /bin/sh
# Starts XVFB in the background.
#
# When this is run, the following files are (over)written in the working directory:
#
# * `xvfb.display`: X server display number
# * `xvfb.pid`: `Xvfb` process id
# * `xvfb.stdout`: stdout output
# * `xvfb.stderr`: stderr output
#
# Adapted from <https://stackoverflow.com/questions/2520704/find-a-free-x11-display-number>
# I have tried wrapping this in `/sbin/start-stop-daemon` but never managed to get Xvfb to
# start.
exec 3>xvfb.display
/usr/bin/nohup /usr/bin/Xvfb -displayfd 3 \
                             -screen 0 1280x1024x24+32 \
                             -pixdepths 3 27 32 \
                             -ac \
                             +extension GLX \
                             +render \
                             -noreset \
                             > xvfb.stdout \
                             2> xvfb.stderr \
                             &
echo $! > xvfb.pid
