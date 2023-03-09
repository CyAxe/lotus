<p align="center">
<img src="https://user-images.githubusercontent.com/45688522/222855427-5a328880-1ded-4adf-ac8e-35d0ba1c8cdd.png" width="470px" >
<!-- img src="https://user-images.githubusercontent.com/45688522/222831284-09899d3b-322b-4215-ba99-8294d4bf8a9b.png" width="170px" -->
</p>
Lotus is an automation system for Dynamic Application Security Testing (DAST) that simplifies web security scripting by providing a Lua API with numerous functions that can be used to automate web security processes in just a few lines of code


Our aim is to make security scripting more efficient by providing libraries and functions to ensure that no critical security features are missed in most security scripting cases.

At present, Lotus is in beta version (0.4), meaning that several ideas are yet to be implemented. However, 
we welcome any contributions that can enable us to complete the project as quickly as possible and move into stable versions
If you have any questions, please refer to the Github repository issues page or join our Discord server (https://discord.gg/nBYDPTzjSq)

## :rocket: Installation
You can install Lotus from the source code by running the following commands:
```bash
$ apt install libssl-dev pkg-config gcc git -y
$ cargo install --git=https://github.com/rusty-sec/lotus/
```
Before running the command, ensure that you have installed the openssl-dev package. If you encounter any challenges while compiling, please open an issue on our Github repository for assistance. Alternatively, you can download the binary file suitable for your operating system and run it directly.

Next, you will need to download the Lua scripts from our Github repository and run the following command:



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
