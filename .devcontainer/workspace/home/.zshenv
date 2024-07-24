ZDOTDIR="$HOME/.config/zsh"

. /usr/local/etc/environment.d/30-java
. /usr/local/etc/environment.d/31-maven
. /usr/local/etc/environment.d/40-rust
. /usr/local/etc/environment.d/50-swift

# Set PATH so it includes user's private bin if it exists
if [ -d "$HOME/.local/bin" ] ; then
  PATH="$HOME/.local/bin:$PATH"
fi
