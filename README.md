attache
=======

An assistant for managing your *NIX dotfiles with source-control.

The goal is basically a bash re-implementation of 
[Briefcase](https://github.com/jim/briefcase) but probably with some
extended features.  We'll see how it goes.


Feature Checklist/Roadmap
-------------------------

- [x] init   - Create a new attache
- [x] add    - Add a file/folder to the attache
- [ ] redact - Add a file with sensitive information, do complicated
  stuff to make it safe(r)
- [ ] status - Currently, just does `git status` in the attache dir,
  should give info about other stuff like what files haven't been linked
- [ ] git    - Run arbitrary git commands in the attache dir
- [ ] open   - (?) Move user into the attache dir for down and dirty work
- [ ] link   - Generate symlinks for all files in your attache dir
  into home
- [ ] fetch - Get a remote git repository and set it up as the
  contents of your attache dir

Thoughts n' stuff:

We've got a whole
[themey-wemey](http://www.google.com/url?sa=i&rct=j&q=&esrc=s&source=images&cd=&cad=rja&uact=8&docid=qh5-8lwMq34inM&tbnid=Tart29iHCeBwFM:&ved=0CAUQjRw&url=http%3A%2F%2Ftallguyalec.buzznet.com%2Fuser%2Fjournal%2F17376075%2Fcountdown-50th-anniversary-24-modern%2F&ei=8jNkU_nvFYjyoATJj4D4DA&bvm=bv.65788261,d.cGU&psig=AFQjCNEXZEifTXzIf_w8-xG7x-dRywn_PA&ust=1399162225760052)
thing going on. Any suggestions or problems with the names should take
this into consideration ;)
