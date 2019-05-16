import("../crate/pkg").then(module => {

  const outputEncoded = e => {
    document.getElementById("output-safe").innerHTML = module.encode_html(e.target.value)
  }

  const outputUnencoded = e => {
    document.getElementById("output-unsafe").innerHTML = e.target.value
  }

  document.getElementById("input-safe").addEventListener("blur", outputEncoded)
  document.getElementById("input-unsafe").addEventListener("blur", outputUnencoded)


  // benchmarking shite
  function getFile(event) {
  	const input = event.target
    if ('files' in input && input.files.length > 0) {
  	  return input.files[0]
    }
  }

  function placeFileContent(target, file) {
  	readFileContent(file).then(content => {
    	target.value = content
    }).catch(error => console.log(error))
  }

  function readFileContent(file) {
  	const reader = new FileReader()
    return new Promise((resolve, reject) => {
      reader.onload = event => resolve(event.target.result)
      reader.onerror = error => reject(error)
      reader.readAsText(file)
    })
  }

  document.getElementById('input-file').addEventListener('change', benchmark)
  async function benchmark(e) {

    let fileContent = await readFileContent(getFile(e))

    function naiveEscapeHTML (fileContent) {
      return fileContent
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/\"/g, '&quot;')
        .replace(/\'/g, '&#39;')
        .replace(/\//g, '&#x2F;')
    }
    let startRust = new Date()
    let placeholderRust = module.encode_html(fileContent)
    let endRust = new Date()
    let startJs = new Date()
    let placeholderJs = naiveEscapeHTML(fileContent)
    let endJs = new Date()

    console.log("JS naive escaping: ", endJs-startJs)
    console.log("Rust through wasm: ", endRust-startRust)
  }
})
