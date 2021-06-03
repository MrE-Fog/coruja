import React from 'react';
import './App.css';

interface CertificatePayload {
    subject: CertificateSubjectPayload;
    issuer: CertificateIssuerPayload;
    notBefore: string,
    notAfter: string,
    expiresInDays: number,
    serialNumber: string,
    pem: string,
}

interface CertificateSubjectPayload {
    commonName: string;
}

interface CertificateIssuerPayload {
    commonName: string;
}

function App() {
    const [host, setHost] = React.useState('');

    const onClickDownloadCertsButton = async () => {
        console.time('download-certs');
        // TODO parameterize this URL
        // TOOD create a service for this
        const result = await fetch('/api/certificates?' + new URLSearchParams({ host }), {
            method: 'GET',
            headers: {
                'Accept': 'application/json',
            },
        });
        // const certs: CertificatePayload[] = await result.json();
        await result.json();
        console.timeEnd('download-certs');
    };

    return (
        <div className="App">
            <label htmlFor="host" />
            <input id="host-input" placeholder="" onChange={event => setHost(event.target.value)} />
            <button id="download-certs-button" onClick={onClickDownloadCertsButton}>
                Download Certs
            </button>
        </div>
    );
}

export default App;
