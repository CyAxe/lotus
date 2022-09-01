function main(url) 
    found = {}
    found["valid"] = false
    local new_url = urljoin(url,"/secured/phpinfo.php")
    local resp = send_req(new_url)
    if resp.url:GetStrOrNil() then 
        local body = resp.body:GetStrOrNil()
        local status = resp.status:GetStrOrNil()
        if ( string.find(body,"PHP Extension") and string.find(body,"PHP Version")) then 
            found["url"] = resp.url:GetStrOrNil()
            found["match"] = "/secured/phpinfo.php"
            found["valid"] = true
            found["payload"] = ""
        end
    end
    return found
end
