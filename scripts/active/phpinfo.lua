report = {}
valid = false
function main(url) 
    local new_url = urljoin(url,"/secured/phpinfo.php")
    local resp = send_req(new_url)
    if resp.body:GetStrOrNil() then 
        local body = resp.body:GetStrOrNil()
        local status = resp.status:GetStrOrNil()
        if ( string.find(body,"PHP Extension") and string.find(body,"PHP Version")) then 
            report["url"] = urljoin(url,"/secured/phpinfo.php")
            report["match"] = "/secured/phpinfo.php"
            report["payload"] = ""
            valid = true
        end
    end
    return report
end


main(TARGET_URL)
