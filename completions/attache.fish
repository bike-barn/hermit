function __fish_attache_needs_command
  set cmd (commandline -opc)
  if [ (count $cmd) -eq 1 -a $cmd[1] = 'attache' ]
    return 0
  end
  return 1
end

function __fish_attache_using_command
  set cmd (commandline -opc)
  if [ (count $cmd) -gt 1 ]
    if [ $argv[1] = $cmd[2] ]
      return 0
    end
  end
  return 1
end

complete -f -c attache -n '__fish_attache_needs_command' -a '(attache commands)'
for cmd in (attache commands)
  complete -f -c attache -n "__fish_attache_using_command $cmd" -a "(attache completions $cmd)"
end