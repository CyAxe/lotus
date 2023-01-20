local function send_report(url) 
    CveReport:setName("CVE-555-555")
    CveReport:setDescription("http://example.com")
    CveReport:setRisk("high")
    CveReport:setUrl(url)

end


function main()
    local response_body = "Hello World"
    local raw_response = "200 Ok\nServer: Nginx\n\n\nHello World"
    --[[
        1 => ReportMatchers::RawResponse(matcher_string),
        2 => ReportMatchers::ResposneHeaders(matcher_string),
        3 => ReportMatchers::ResponseBody(matcher_string),
        _ => ReportMatchers::General(matcher_string),

    ]]--
    CveReport:addMatcher(response_body,3)
    CveReport:addMatcher(raw_response,1)
    send_report(HttpMessage:getUrl())
end
