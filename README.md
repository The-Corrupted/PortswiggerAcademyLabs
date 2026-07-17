#Portswigger Academy Lab Solutions

This is a collection of scripts I wrote to solve the Portswigger Academy labs for the various modules. The purpose of this repository is to keep a record
of the scripts in case I need them later for the BSCP exam and to provide my reasoning and steps I took to solve the labs.

##Why turbo intruder?

Burp Suite Community Edition is the free edition of Burp Suite that I am using to solve the labs. Burp Suite Pro is a $500 annual subscription which is currently
too expensive for me. I could start a free trial, however it's not obvious to me when I'll have completed enough of the modules and labs to qualify for the
BSCP exam so I'm saving the trial until I'm ready to finish the remaining labs that require pro features ( Collaborator ) and then apply for the exam.

Normally the way you would solve many of the injection labs or brute-force labs would be to use intruder to run sniper, cluster bomb, pitchfork or battering ram attachs, but intruder on community edition implements request throttling to push users towards buying a license. This results in labs that should only take a few minutes to complete turning into hour(s) long ordeals where you need to carefully monitor progress to ensure you're at the computer when it finishes otherwise inactivity will trigger the lab instance to go down and you need to start over. Turbo intruder is an extension for Burp Suite that bypasses community edition
rate-limiting, allowing you to complete labs on a free license in a reasonable amount of time with the main trade-off being that you need to write the python
for each of your attacks rather than using the simpler GUI interface intruder provides. Because I do not want to wait all day for attacks to finish and
am familiar with python, I have opted to use turbo intruder for all attacks that need it and save my scripts in a repository so I can reference them later.

##Do all labs use intruder?

No. Some of the directories here will not contain a turbo intruder script because intruder capabilities were not required to complete the lab.
These directories will just contain a README.md file that explains the lab and provides the steps I took to solve it.
