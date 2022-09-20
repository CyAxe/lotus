REPORT = {}
VALID = false
STOP_AFTER_MATCH = true
THREADS = 1

function payloads_gen(url)
    new_url = {urljoin(url,"/secured/phpinfo.php")}
    return new_url
end


function main(param,url) 
    local resp = send_req(url)
    if resp.body:GetStrOrNil() then 
        local body = resp.body:GetStrOrNil()
        if ( string.find(body,"PHP Extension") and string.find(body,"PHP Version")) then 
            REPORT["url"] = url
            REPORT["match"] = "/secured/phpinfo.php"
            REPORT["payload"] = ""
            VALID = true
            println(string.format("PHPINFO: %s",url))
        end
    end
    return REPORT
end

