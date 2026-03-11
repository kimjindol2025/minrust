# MinRust Tokenizer - Lexical Analysis
# 소스 코드를 토큰 스트림으로 변환

module MinRustTokenizer

export TokenType, Token, tokenize, is_keyword

# ============================================================
# Token Types
# ============================================================

@enum TokenType begin
    # 키워드
    KW_FN
    KW_LET
    KW_CONST
    KW_MUT
    KW_IF
    KW_ELSE
    KW_FOR
    KW_WHILE
    KW_LOOP
    KW_MATCH
    KW_STRUCT
    KW_IMPL
    KW_TRAIT
    KW_PUB
    KW_USE
    KW_MOD
    KW_CRATE
    KW_AS
    KW_DYN
    KW_UNSAFE
    KW_STATIC
    KW_ASYNC
    KW_AWAIT
    KW_MOVE
    KW_REF
    KW_IN
    KW_WHERE
    KW_TYPE
    KW_ENUM
    KW_RETURN
    KW_BREAK
    KW_CONTINUE
    KW_TRUE
    KW_FALSE

    # 리터럴
    INT_LITERAL
    FLOAT_LITERAL
    STRING_LITERAL
    CHAR_LITERAL
    BOOL_LITERAL

    # 식별자
    IDENTIFIER

    # 펑크추에이션
    LPAREN          # (
    RPAREN          # )
    LBRACE          # {
    RBRACE          # }
    LBRACKET        # [
    RBRACKET        # ]
    COMMA           # ,
    SEMICOLON       # ;
    COLON           # :
    COLONCOLON      # ::
    DOT             # .
    DOTDOT          # ..
    ARROW           # ->
    FAT_ARROW       # =>
    QUESTION        # ?

    # 연산자
    PLUS            # +
    MINUS           # -
    STAR            # *
    SLASH           # /
    PERCENT         # %
    CARET           # ^
    AMPERSAND       # &
    PIPE            # |
    TILDE           # ~
    NOT             # !
    ASSIGN          # =
    LT              # <
    GT              # >
    LE              # <=
    GE              # >=
    EQ              # ==
    NE              # !=
    AND             # &&
    OR              # ||
    LSHIFT          # <<
    RSHIFT          # >>
    PLUS_ASSIGN     # +=
    MINUS_ASSIGN    # -=
    STAR_ASSIGN     # *=
    SLASH_ASSIGN    # /=

    # 특수
    EOF_TOKEN
    UNKNOWN
end

# ============================================================
# Token Structure
# ============================================================

struct Token
    type::TokenType
    value::Union{String, Int, Float, Nothing}
    line::Int
    column::Int
end

# ============================================================
# Keyword Map
# ============================================================

const KEYWORDS = Dict(
    "fn" => KW_FN,
    "let" => KW_LET,
    "const" => KW_CONST,
    "mut" => KW_MUT,
    "if" => KW_IF,
    "else" => KW_ELSE,
    "for" => KW_FOR,
    "while" => KW_WHILE,
    "loop" => KW_LOOP,
    "match" => KW_MATCH,
    "struct" => KW_STRUCT,
    "impl" => KW_IMPL,
    "trait" => KW_TRAIT,
    "pub" => KW_PUB,
    "use" => KW_USE,
    "mod" => KW_MOD,
    "crate" => KW_CRATE,
    "as" => KW_AS,
    "dyn" => KW_DYN,
    "unsafe" => KW_UNSAFE,
    "static" => KW_STATIC,
    "async" => KW_ASYNC,
    "await" => KW_AWAIT,
    "move" => KW_MOVE,
    "ref" => KW_REF,
    "in" => KW_IN,
    "where" => KW_WHERE,
    "type" => KW_TYPE,
    "enum" => KW_ENUM,
    "return" => KW_RETURN,
    "break" => KW_BREAK,
    "continue" => KW_CONTINUE,
    "true" => KW_TRUE,
    "false" => KW_FALSE,
)

# ============================================================
# Helper Functions
# ============================================================

function is_keyword(word::String)::Bool
    return haskey(KEYWORDS, word)
end

function is_digit(c::Char)::Bool
    return '0' <= c <= '9'
end

function is_alpha(c::Char)::Bool
    return ('a' <= c <= 'z') || ('A' <= c <= 'Z') || c == '_'
end

function is_alphanumeric(c::Char)::Bool
    return is_alpha(c) || is_digit(c)
end

function is_whitespace(c::Char)::Bool
    return c in (' ', '\t', '\n', '\r')
end

# ============================================================
# Tokenizer Structure
# ============================================================

mutable struct Tokenizer
    input::String
    pos::Int
    line::Int
    column::Int
    tokens::Vector{Token}
end

function Tokenizer(input::String)
    return Tokenizer(input, 1, 1, 1, Token[])
end

# ============================================================
# Core Tokenization Functions
# ============================================================

function tokenize(input::String)::Vector{Token}
    tokenizer = Tokenizer(input)

    while tokenizer.pos <= length(tokenizer.input)
        scan_token!(tokenizer)
    end

    push!(tokenizer.tokens, Token(EOF_TOKEN, nothing, tokenizer.line, tokenizer.column))
    return tokenizer.tokens
end

function peek(tokenizer::Tokenizer)::Char
    if tokenizer.pos > length(tokenizer.input)
        return '\0'
    end
    return tokenizer.input[tokenizer.pos]
end

function peek_next(tokenizer::Tokenizer)::Char
    if tokenizer.pos + 1 > length(tokenizer.input)
        return '\0'
    end
    return tokenizer.input[tokenizer.pos + 1]
end

function advance!(tokenizer::Tokenizer)::Char
    c = peek(tokenizer)
    tokenizer.pos += 1
    if c == '\n'
        tokenizer.line += 1
        tokenizer.column = 1
    else
        tokenizer.column += 1
    end
    return c
end

function add_token!(tokenizer::Tokenizer, type::TokenType, value::Union{String, Int, Float, Nothing} = nothing)
    token = Token(type, value, tokenizer.line, tokenizer.column)
    push!(tokenizer.tokens, token)
end

function scan_token!(tokenizer::Tokenizer)
    c = advance!(tokenizer)

    # 공백 스킵
    if is_whitespace(c)
        return
    end

    # 주석
    if c == '/'
        if peek(tokenizer) == '/'
            # 라인 주석
            while peek(tokenizer) != '\n' && peek(tokenizer) != '\0'
                advance!(tokenizer)
            end
            return
        elseif peek(tokenizer) == '*'
            # 블록 주석
            advance!(tokenizer)  # consume '*'
            while true
                if peek(tokenizer) == '\0'
                    return
                end
                if peek(tokenizer) == '*' && peek_next(tokenizer) == '/'
                    advance!(tokenizer)
                    advance!(tokenizer)
                    break
                end
                advance!(tokenizer)
            end
            return
        end
    end

    # 문자
    if c == '('
        add_token!(tokenizer, LPAREN)
        return
    elseif c == ')'
        add_token!(tokenizer, RPAREN)
        return
    elseif c == '{'
        add_token!(tokenizer, LBRACE)
        return
    elseif c == '}'
        add_token!(tokenizer, RBRACE)
        return
    elseif c == '['
        add_token!(tokenizer, LBRACKET)
        return
    elseif c == ']'
        add_token!(tokenizer, RBRACKET)
        return
    elseif c == ','
        add_token!(tokenizer, COMMA)
        return
    elseif c == ';'
        add_token!(tokenizer, SEMICOLON)
        return
    elseif c == '?'
        add_token!(tokenizer, QUESTION)
        return
    elseif c == ':'
        if peek(tokenizer) == ':'
            advance!(tokenizer)
            add_token!(tokenizer, COLONCOLON)
        else
            add_token!(tokenizer, COLON)
        end
        return
    elseif c == '.'
        if peek(tokenizer) == '.'
            advance!(tokenizer)
            add_token!(tokenizer, DOTDOT)
        else
            add_token!(tokenizer, DOT)
        end
        return
    elseif c == '+'
        if peek(tokenizer) == '='
            advance!(tokenizer)
            add_token!(tokenizer, PLUS_ASSIGN)
        else
            add_token!(tokenizer, PLUS)
        end
        return
    elseif c == '-'
        if peek(tokenizer) == '>'
            advance!(tokenizer)
            add_token!(tokenizer, ARROW)
        elseif peek(tokenizer) == '='
            advance!(tokenizer)
            add_token!(tokenizer, MINUS_ASSIGN)
        else
            add_token!(tokenizer, MINUS)
        end
        return
    elseif c == '*'
        if peek(tokenizer) == '='
            advance!(tokenizer)
            add_token!(tokenizer, STAR_ASSIGN)
        else
            add_token!(tokenizer, STAR)
        end
        return
    elseif c == '/'
        if peek(tokenizer) == '='
            advance!(tokenizer)
            add_token!(tokenizer, SLASH_ASSIGN)
        else
            add_token!(tokenizer, SLASH)
        end
        return
    elseif c == '%'
        add_token!(tokenizer, PERCENT)
        return
    elseif c == '='
        if peek(tokenizer) == '='
            advance!(tokenizer)
            add_token!(tokenizer, EQ)
        elseif peek(tokenizer) == '>'
            advance!(tokenizer)
            add_token!(tokenizer, FAT_ARROW)
        else
            add_token!(tokenizer, ASSIGN)
        end
        return
    elseif c == '!'
        if peek(tokenizer) == '='
            advance!(tokenizer)
            add_token!(tokenizer, NE)
        else
            add_token!(tokenizer, NOT)
        end
        return
    elseif c == '<'
        if peek(tokenizer) == '<'
            advance!(tokenizer)
            add_token!(tokenizer, LSHIFT)
        elseif peek(tokenizer) == '='
            advance!(tokenizer)
            add_token!(tokenizer, LE)
        else
            add_token!(tokenizer, LT)
        end
        return
    elseif c == '>'
        if peek(tokenizer) == '>'
            advance!(tokenizer)
            add_token!(tokenizer, RSHIFT)
        elseif peek(tokenizer) == '='
            advance!(tokenizer)
            add_token!(tokenizer, GE)
        else
            add_token!(tokenizer, GT)
        end
        return
    elseif c == '&'
        if peek(tokenizer) == '&'
            advance!(tokenizer)
            add_token!(tokenizer, AND)
        else
            add_token!(tokenizer, AMPERSAND)
        end
        return
    elseif c == '|'
        if peek(tokenizer) == '|'
            advance!(tokenizer)
            add_token!(tokenizer, OR)
        else
            add_token!(tokenizer, PIPE)
        end
        return
    elseif c == '^'
        add_token!(tokenizer, CARET)
        return
    elseif c == '~'
        add_token!(tokenizer, TILDE)
        return
    end

    # 문자열
    if c == '"'
        scan_string!(tokenizer, '"')
        return
    end

    # 문자
    if c == '\''
        scan_char!(tokenizer)
        return
    end

    # 숫자
    if is_digit(c)
        tokenizer.pos -= 1
        tokenizer.column -= 1
        scan_number!(tokenizer)
        return
    end

    # 식별자 / 키워드
    if is_alpha(c)
        tokenizer.pos -= 1
        tokenizer.column -= 1
        scan_identifier!(tokenizer)
        return
    end

    add_token!(tokenizer, UNKNOWN)
end

function scan_string!(tokenizer::Tokenizer, delimiter::Char)
    start_line = tokenizer.line
    start_column = tokenizer.column
    value = ""

    while peek(tokenizer) != delimiter && peek(tokenizer) != '\0'
        if peek(tokenizer) == '\\'
            advance!(tokenizer)
            next = advance!(tokenizer)
            if next == 'n'
                value *= '\n'
            elseif next == 't'
                value *= '\t'
            elseif next == 'r'
                value *= '\r'
            elseif next == '\\'
                value *= '\\'
            elseif next == '"'
                value *= '"'
            elseif next == '\''
                value *= '\''
            else
                value *= next
            end
        else
            value *= advance!(tokenizer)
        end
    end

    if peek(tokenizer) == delimiter
        advance!(tokenizer)  # closing delimiter
    end

    add_token!(tokenizer, STRING_LITERAL, value)
end

function scan_char!(tokenizer::Tokenizer)
    value = ""

    if peek(tokenizer) == '\\'
        advance!(tokenizer)
        next = advance!(tokenizer)
        if next == 'n'
            value = "\n"
        elseif next == 't'
            value = "\t"
        elseif next == '\\'
            value = "\\"
        else
            value = string(next)
        end
    else
        value = string(advance!(tokenizer))
    end

    if peek(tokenizer) == '\''
        advance!(tokenizer)
    end

    add_token!(tokenizer, CHAR_LITERAL, value)
end

function scan_number!(tokenizer::Tokenizer)
    value = ""

    while is_digit(peek(tokenizer))
        value *= advance!(tokenizer)
    end

    # 부동소수
    if peek(tokenizer) == '.' && is_digit(peek_next(tokenizer))
        value *= advance!(tokenizer)  # consume '.'
        while is_digit(peek(tokenizer))
            value *= advance!(tokenizer)
        end
        add_token!(tokenizer, FLOAT_LITERAL, parse(Float64, value))
    else
        add_token!(tokenizer, INT_LITERAL, parse(Int, value))
    end
end

function scan_identifier!(tokenizer::Tokenizer)
    value = ""

    while is_alphanumeric(peek(tokenizer))
        value *= advance!(tokenizer)
    end

    token_type = get(KEYWORDS, value, IDENTIFIER)

    if token_type == IDENTIFIER
        add_token!(tokenizer, IDENTIFIER, value)
    else
        add_token!(tokenizer, token_type)
    end
end

end  # module MinRustTokenizer
