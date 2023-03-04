# sppg
Secure passphrase generator
For a long time I've relied on `apg`, a random password generator that generates pronounceable random passwords, to create my passwords for me. I've been happy with it so far but there are a few problems with using it.
1. I've found that my limit for secure password that I can memorize in a reasonable amount of time is about 12 characters
2. I can't remember more than a couple of these secure passwords without writing them down somewhere
3. Since it takes time to memorize a new password I am loathe to change a password once I memorize it

None of these problems is the fault of the program. I've gotten around them by using a password manager for every app or site that requires a password. I let the password manager generate a secure, utterly un-memorizable password and just let the password manager enter it for me automatically whenever I need it. Since my password manager has a browser extention, a mobile app, and a desktop app I can be reasonably certain that I will have access to my passwords almost anywhere I may be. This leaves me with only 2 passwords to remember and aren't written down anywhere: my desktop password and the master password for my password manager. This compromise has worked out reasonably well for me so far. This leaves me with only one last problem, rotating my passwords. In addition, since I now store all my passwords in the password manager my master password should probably be more secure than a random twelve char password. 

Enter [diceware](https://theworld.com/~reinhold/diceware.html). Diceware is a method for generating very secure yet easily memorizable passwords. The apeal for me is that it allows me to remove all the things that make remembering a password difficult for me: special characters and upper case letters. Now my passwords can just be 4 or 5 easily memorizable lower case words. I just needed to tackle one more problem. I need a command line program to generate them for me. That tools is sppg.

#Installation

#Usage

