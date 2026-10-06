pub fn generate() -> String {
    String::from(
        "_smart_term_up_arrow() {\n\
            ((_SMART_HIST_INDEX++))\n\
            local cmd=$(smart hist-nav $_SMART_HIST_INDEX)\n\
            if [[ -n \"$cmd\" ]]; then\n\
                BUFFER=\"$cmd\"\n\
                CURSOR=${#BUFFER}\n\
            else\n\
                ((_SMART_HIST_INDEX--))\n\
            fi\n\
        }\n\
        _smart_term_down_arrow() {\n\
            if [[ $_SMART_HIST_INDEX -gt 1 ]]; then\n\
                ((_SMART_HIST_INDEX--))\n\
                BUFFER=$(smart hist-nav $_SMART_HIST_INDEX)\n\
                CURSOR=${#BUFFER}\n\
            elif [[ $_SMART_HIST_INDEX -eq 1 ]]; then\n\
                _SMART_HIST_INDEX=0\n\
                BUFFER=\"\"\n\
                CURSOR=0\n\
            fi\n\
        }\n\
        zle -N _smart_term_up_arrow\n\
        zle -N _smart_term_down_arrow\n\
        bindkey '^[[A' _smart_term_up_arrow\n\
        bindkey '^[[B' _smart_term_down_arrow\n\
        autoload -Uz add-zsh-hook\n\
        _smart_term_precmd() {\n\
            _SMART_HIST_INDEX=0\n\
            smart hook pre-cmd $?\n\
            export SMART_PROMPT=\"$(smart prompt)\"\n\
        }\n\
        _smart_term_preexec() {\n\
            smart hook pre-exec \"$1\"\n\
        }\n\
        add-zsh-hook precmd _smart_term_precmd\n\
        add-zsh-hook preexec _smart_term_preexec\n\
        if [[ \"$PROMPT\" != *\"\\$SMART_PROMPT\"* ]]; then\n\
            PROMPT=\"%F{yellow}\\$SMART_PROMPT%f$PROMPT\"\n\
        fi\n"
    )
}
