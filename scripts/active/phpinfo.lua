report = {}
valid = false
function main(url) 
    local new_url = urljoin(url,"/secured/phpinfo.php")
    local resp = send_req(new_url)
    if resp.body:GetStrOrNil() then 
        local body = resp.body:GetStrOrNil()
        local status = resp.status:GetStrOrNil()
        if ( string.find(body,"PHP Extension") and string.find(body,"PHP Version")) then 
            report["url"] = new_url
            report["match"] = "/secured/phpinfo.php"
            valid = true
            report["payload"] = ""
        end
    end
    return report
end


main(TARGET_URL)
