import resources from "./mvp/fake-data"

global css html
	ff:sans

global css body
	m: 0

tag app
	resources = [ {title: "@alexanderalemayhu | Linktree", resource: "https://linktr.ee/alexanderalemayhu", format: "http", children: null}]

	def onToggleClicked(event)
		const toggle = event.target
		console.log('is toggle open?', toggle.open)
		if toggle.open
			const res = resources[0]
			console.log('fetching', res.resource)
			console.log('')
			const response = await window.fetch(res.resource)
			const page = await response.text()
			console.log('page21', page)


	<self [bg: red d:block w: 100vw h: 100vh]>
		<div [d: flex p: 0 2rem]>
			<div [bg: blue4 h: 100vh w: 30vw]> 
					for res in resources
						<details @toggle=onToggleClicked> 
							<summary> <a [href: '#'] [d: flex p: 0 2rem]> res.title
							"everything else"
			<div [bg: purple4 h: 100vh w: 70vw]> 
				<input placeholder="https://google.com"> 
				<button> "+"

imba.mount <app>
