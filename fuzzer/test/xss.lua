

function report(url) 
    NewReport:setRisk("low")
    NewReport:setUrl(url)
end

function main(url)
    report(url)
    Reports:addReport(NewReport)
end
