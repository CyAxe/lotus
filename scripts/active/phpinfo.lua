REPORT = {}
VALID = false

local function main(url) 
    local new_url = urljoin(url,"/secured/phpinfo.php")
    local resp = send_req(new_url)
    if resp.body:GetStrOrNil() then 
        local body = resp.body:GetStrOrNil()
        if ( string.find(body,"PHP Extension") and string.find(body,"PHP Version")) then 
            REPORT["url"] = urljoin(url,"/secured/phpinfo.php")
            REPORT["match"] = "/secured/phpinfo.php"
            REPORT["payload"] = ""
            VALID = true
        end
    end
end


main(TARGET_URL)
