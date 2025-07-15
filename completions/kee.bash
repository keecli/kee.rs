# Bash completion for kee
_kee_completion() {
  local cur prev opts
  COMPREPLY=()
  cur="${COMP_WORDS[COMP_CWORD]}"
  prev="${COMP_WORDS[COMP_CWORD-1]}"

  case ${COMP_CWORD} in
    1)
      opts="add use ls current rm help"
      COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
      return 0
      ;;
    2)
      case "${prev}" in
        use|rm)
          # Get account names dynamically
          local accounts=$(${COMP_WORDS[0]} ls --names 2>/dev/null)
          COMPREPLY=( $(compgen -W "${accounts}" -- "${cur}") )
          return 0
          ;;
        ls)
          # Complete ls command flags
          opts="--names --help"
          COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
          return 0
          ;;
        *)
          ;;
      esac
      ;;
    *)
      ;;
  esac
}

complete -F _kee_completion kee
