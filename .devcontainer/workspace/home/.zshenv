ZDOTDIR="$HOME/.config/zsh"

export JAVA_HOME=/usr/local/openjdk
export MAVEN_HOME=/usr/local/maven
export CARGO_HOME=/usr/local/cargo
export RUSTUP_HOME=/usr/local/rustup

PATH="$CARGO_HOME/bin:$MAVEN_HOME/bin:$JAVA_HOME/bin:$PATH"

# set PATH so it includes user's private bin if it exists
if [ -d "$HOME/.local/bin" ] ; then
  PATH="$HOME/.local/bin:$PATH"
fi
