
async function init() {

    let rustApp = null

    try {
        rustApp = await import('../pkg')
    } catch(e) {
        console.error(e)
        return;
    }
    // console.log('rustApp:', rustApp)

    const input = document.getElementById('upload')
    const fileReader = new FileReader()

    fileReader.onloadend = () => {
        let base64 = fileReader.result.replace(
            /^data:image\/(png|jpg|jpeg);base64,/, ''
        )
        rustApp.grayscale(base64)
    }

    input.addEventListener('change', () => {
        fileReader.readAsDataURL(input.files[0])
    })
}

init()
