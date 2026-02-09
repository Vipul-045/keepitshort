const BACKEND = "http://localhost:3001" ; 

export async function short_url(og_url: String) {
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