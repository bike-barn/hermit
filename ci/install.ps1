# This script handles fixing Rust builds with mingw

If ($Env:TARGET -Like '*-gnu') {
  $Env:MSYS_BINDIR='C:\msys64\usr\bin'
  If ($Env:TARGET -Like 'i686-*') {
      $Env:MSYS_BITS='32'
  } ElseIf ($Env:TARGET -Like 'x86_64-*') {
      $Env:MSYS_BITS='64'
  }
}
