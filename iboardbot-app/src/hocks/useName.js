import { useEffect, useRef, useState } from "react";

export function useName() {
    const [name, setName] = useState("");
    const [error, setError] = useState(null);
    const isFirstInput = useRef(true);
  
    useEffect(() => {
      //para saber si es la primera vez del input
      if (isFirstInput.current) {
        isFirstInput.current = name === "";
        return;
      }
      if (name === "") {
        setError("No se puede jugar sin nombre 😭");
        return;
      }
      if (name.match(/^\d+$/)) {
        setError("No puedes ponerte de nombre solo de números");
        return;
      }
      if (name.length < 3) {
        setError("El nombre debe tener al menos 3 caracteres");
        return;
      }
      
      setError(null);
    }, [name,error]);
    return { name, setName, error };
  }