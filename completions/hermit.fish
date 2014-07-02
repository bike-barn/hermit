function __fish_hermit_needs_command
  set cmd (commandline -opc)
  if [ (count $cmd) -eq 1 -a $cmd[1] = 'hermit' ]
    return 0
  end
  return 1
end

function __fish_hermit_using_command
  set cmd (commandline -opc)
  if [ (count $cmd) -gt 1 ]
    if [ $argv[1] = $cmd[2] ]
      return 0
    end
  end
  return 1
end

complete -f -c hermit -n '__fish_hermit_needs_command' -a '(hermit commands)'
for cmd in (hermit commands)
  complete -f -c hermit -n "__fish_hermit_using_command $cmd" -a "(hermit completions $cmd)"
end
