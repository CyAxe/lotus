<p align="center">
<img src="https://user-images.githubusercontent.com/45688522/222855100-a4ac6087-464b-4df9-8fca-f0e5bf2d951a.png" width="470px" >
<!-- img src="https://user-images.githubusercontent.com/45688522/222831284-09899d3b-322b-4215-ba99-8294d4bf8a9b.png" width="170px" -->
</p>
Using Lotus' Lua API, you can automate your own web security module in the shortest amount of time by discovering, scanning, and reporting in just a few lines of code

Currently, we are working hard to write libraries and functions in most cases to ensure that you do not miss any functions with Lotus. Our mission is to make security scripting easier and faster by providing our Lua API with many functions you will need in most security scripting cases. 

As of right now, we are still in beta version (0.4), which means that many ideas have not been implemented into that project

 Consequently, any contribution you are able to make to this project will enable us to complete it as quickly as possible and move into stable versions sooner rather than later. If you have any further questions, please view the github repository issues page or join our Discord server (https://discord.gg/nBYDPTzjSq).

### :rocket: Installation 
It can be built from source, but ensure that you install the package `openssl-dev` before running this command

```bash
$ apt install libssl-dev pkg-config gcc git -y
$ cargo install --git=https://github.com/rusty-sec/lotus/
```
We encourage you to open an issue on our github repository if you are having difficulties compiling it and would like some assistance
You can also download the binary file based on your operating system and run it directly



You will then need to download the lua scripts from our [github repository](https://github.com/rusty-sec/lotus-scripts) and run the following command
but rembmer do use the scripts from the offical repo if you want to use any scirpt from other repos use it on your risk 

```
$ echo http://testphp.vulnweb.com/listproducts.php?cat=1 | lotus urls lotus-scripts/active -o test_out.json
```

![image](https://user-images.githubusercontent.com/45688522/202260525-46caeaeb-8687-4723-a406-aea30e0ea9c6.png)

```bash
khaled@Home ~/work/code/lotus-scripts]$ echo "http://localhost:5000/?name=2" | lotus urls ~/work/code/lotus-scripts/active/ -o out -v
[INFO] URLS: 1
[INFO] HOSTS: 1
[INFO] PATHS: 1

[+] Template Injection on: http://localhost:5000/?name=2lot%7B%7B2*2%7D%7Dus
[#] SCAN TYPE: VULN
[#] Description: https://owasp.org/www-project-web-security-testing-guide/v41/4-Web_Application_Security_Testing/07-Input_Validation_Testing
/18-Testing_for_Server_Side_Template_Injection
[#] Vulnerable Parameter: name
[#] Risk: high
[#] Used Payload: lot{{2*2}}us
[#] Matching Pattern: lot4us
#--------------------------------------------------#
```


### :book: Documentation
you can find lotus docs in docs/*.md dir but if you want a simple one (web version) you can visit https://lotus.knas.me
