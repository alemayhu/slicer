global css html
	ff:sans

global css body
	m: 0

tag app
	collections = [ {title: "@alexanderalemayhu | Linktree", resource: "https://linktr.ee/alexanderalemayhu", format: "http"}]

	<self [bg: red d:block w: 100vw h: 100vh]>
		<div [d: flex p: 0 2rem]>
			<div [bg: blue4 h: 100vh w: 30vw]> 
				<ul [list-style: none]>
					for item in collections 
						<li> 
							<a [href: '#'] [d: flex p: 0 2rem]> item.title
			<div [bg: purple4 h: 100vh w: 70vw]> 
				<input placeholder="https://google.com"> 
				<button> "+"

imba.mount <app>
