const BACKEND = "https://keepitshort-46ze.onrender.com" ; 

export async function short_url(og_url: string) {
    const res = await fetch(`${BACKEND}/`,
        {
            method : "POST",
            headers : {"Content-Type": "application/json"},
            body: JSON.stringify({og_url})
        }
    )    
    const data = await res.json();

    if(!res.ok){
        throw new Error(data.error);
    }

    return data;
}

export async function get_url(code: string) {
    const res = await fetch(`${BACKEND}/${code}`,
        {
            method: "GET",
            headers : {"Content-Type": "application/json"},
        }
    )
    return res.json();
}