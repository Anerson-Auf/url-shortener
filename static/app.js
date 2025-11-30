document.addEventListener('DOMContentLoaded', async () => {
    const button = document.querySelector('.button');
    const input = document.querySelector('input');
    const copyButton = document.querySelector('.Btn');

    let baseUrl = '';
    try {
        const configResponse = await fetch('/api/config');
        if (configResponse.ok) {
            const config = await configResponse.json();
            baseUrl = config.base_url;
        }
    } catch (error) {
        console.error('Failed to load config:', error);
    }

    if (button) {
        button.addEventListener('click', async (e) => {
            e.preventDefault();
            e.stopPropagation();
            const url = input.value;
            let trimmed = url.trim();
            if (trimmed.length === 0) {
                return;
            }
            if (url) {
                try {
                    const response = await fetch('/api/short-url', {
                        method: 'POST',
                        headers: {
                            'Content-Type': 'application/json',
                        },
                        body: JSON.stringify({ url }),
                    });
                    if (response.ok) {
                        const shortCode = await response.json();
                        const fullUrl = `${baseUrl}/r/${shortCode}`;
                        const textElement = copyButton.querySelector('.text');
                        if (textElement) {
                            textElement.textContent = fullUrl;
                        }
                    } else {
                        console.error('Error:', response.status, response.statusText);
                    }
                } catch (error) {
                    console.error('Error:', error);
                }
            }
        });
    }

    if (copyButton) {
        copyButton.addEventListener('click', () => {
            const textElement = copyButton.querySelector('.text');
            if (textElement && textElement.textContent) {
                navigator.clipboard.writeText(textElement.textContent);
            }
        });
    }
});