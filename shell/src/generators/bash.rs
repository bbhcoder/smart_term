pub fn generate() -> String {
    String::from(
        "_smart_term_up_arrow() {\n\
            ((_SMART_HIST_INDEX++))\n\
            local cmd=$(smart hist-nav $_SMART_HIST_INDEX)\n\
            if [[ -n \"$cmd\" ]]; then\n\
                READLINE_LINE=\"$cmd\"\n\
                READLINE_POINT=${#READLINE_LINE}\n\
            else\n\
                ((_SMART_HIST_INDEX--))\n\
            fi\n\
        }\n\
        _smart_term_down_arrow() {\n\
            if [[ $_SMART_HIST_INDEX -gt 1 ]]; then\n\
                ((_SMART_HIST_INDEX--))\n\
                READLINE_LINE=$(smart hist-nav $_SMART_HIST_INDEX)\n\
                READLINE_POINT=${#READLINE_LINE}\n\
            elif [[ $_SMART_HIST_INDEX -eq 1 ]]; then\n\
                _SMART_HIST_INDEX=0\n\
                READLINE_LINE=\"\"\n\
                READLINE_POINT=0\n\
            fi\n\
        }\n\
        _smart_term_precmd() {\n\
            local exit_code=$?\n\
            _SMART_HIST_INDEX=0\n\
            local raw_hist=$(history 1)\n\
            local cmd=$(echo \"$raw_hist\" | sed 's/^[ \\t]*[0-9]*[ \\t]*//')\n\
            if [[ \"$cmd\" != \"$_SMART_LAST_CMD\" && -n \"$cmd\" ]]; then\n\
                _SMART_LAST_CMD=\"$cmd\"\n\
                smart hook pre-exec \"$cmd\"\n\
            fi\n\
            smart hook pre-cmd $exit_code\n\
            export SMART_PROMPT=\"$(smart prompt)\"\n\
        }\n\
        if [[ \"$PROMPT_COMMAND\" != *\"_smart_term_precmd\"* ]]; then\n\
            PROMPT_COMMAND=\"_smart_term_precmd; $PROMPT_COMMAND\"\n\
        fi\n\
        if [[ \"$PS1\" != *\"\\$SMART_PROMPT\"* ]]; then\n\
            PS1=\"\\[\\e[33m\\]\\$SMART_PROMPT\\[\\e[0m\\]$PS1\"\n\
        fi\n\
        bind -x '\"\\e[A\": _smart_term_up_arrow'\n\
        bind -x '\"\\e[B\": _smart_term_down_arrow'\n"
    )
}
