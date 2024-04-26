export PROMPT="%(?.%F{green}✓.%F{red}%?)%f"             # Green checkmark or red status code.
export PROMPT="$PROMPT %F{green}%n@%m%f"                # User and host names.
export PROMPT="$PROMPT %F{cyan}%4~%f"                   # Current directory.
export PROMPT="$PROMPT %(!.%F{red}#.%F{green}>)%f "     # Red # if root, green > otherwise.

export PROMPT_EOL_MARK="%B%F{yellow}⏎%f%b"              # Yellow carriage return on missing EOL.

export LS_COLORS="di=36:ln=95:ex=32"

alias ls="ls -aF --color=auto"
alias tree="tree -aCF"

HISTFILE=${ZDOTDIR:-$HOME}/.zsh_history
HISTSIZE=2000
SAVEHIST=1000

setopt histignorespace
