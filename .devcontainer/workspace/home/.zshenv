ZDOTDIR="$HOME/.config/zsh"

eval $(/usr/local/libexec/environment_helper)

# Set PATH so it includes user's private bin if it exists
if [ -d "$HOME/.local/bin" ] ; then
  PATH="$HOME/.local/bin:$PATH"
fi
