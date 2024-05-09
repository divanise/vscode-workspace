ZDOTDIR="$HOME/.config/zsh"

JAVA_HOME=/opt/java/openjdk
MAVEN_HOME=/opt/java/maven

PATH="$HOME/.cargo/bin:$MAVEN_HOME/bin:$JAVA_HOME/bin:$PATH"

# set PATH so it includes user's private bin if it exists
if [ -d "$HOME/.local/bin" ] ; then
  PATH="$HOME/.local/bin:$PATH"
fi
