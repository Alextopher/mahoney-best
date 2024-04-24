async function uploadFiles() {
    console.log('Uploading files...');

    const files = document.querySelector('input[type="file"]').files;
    const formData = new FormData();

    for (let i = 0; i < files.length; i++) {
        formData.append('file', files[i]);
    }

    let resp = await fetch('/f/', {
        method: 'POST',
        body: formData,
    });

    if (resp.ok) {
        console.log('Files uploaded successfully!');
    } else {
        console.error('Failed to upload files!', resp.text());
    }
}