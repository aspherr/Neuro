<script>
	import { marked } from "marked";
	let someMarkdown = "# Hello from Markdown! \n This is _really_ just a markdown string! \n\n Using `marked` we can generate a HTML string from it and using Sveltes `@html` tag we can render that HTML easily!"
</script>

{@html marked(someMarkdown)}