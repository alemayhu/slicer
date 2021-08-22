import resources from "../fake-data"
import axios from "axios"
import cheerio from "cheerio"

# if require.main === module

def getPage resource
	const page = await axios.get(resource)
	cheerio.load(page.data)

def main
	console.log('called directly', resources)
	const first = resources[0]
	const page = await getPage(first.resource)

	# console.log('page', page)
	console.log('cherrio', page("title").html())
	# max depth 21
	# const links = 
	page("a").map do |link, elem|
		const href = elem.attribs.href
		const childPage = await getPage(href)
		const title = childPage("title").html()
		console.log( title || "[untitled]", href)

main!
	
