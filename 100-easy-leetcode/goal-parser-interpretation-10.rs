fn interpret(command: String) -> String {
    command.replace("(al)", "al").replace("()", "o")
}
