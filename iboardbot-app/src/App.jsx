import './App.css';
import { useContext, useState, createContext } from 'react';
import { motion } from 'framer-motion';

const ExitoContext = createContext();
// eslint-disable-next-line react/prop-types
function ExitoProvider({ children }) {
  const [exito, setExito] = useState(false);
  return (
    <ExitoContext.Provider value={[exito, setExito]}>
      {children}
    </ExitoContext.Provider>
  );
}

function Form() {
  const [exito, setExito] = useContext(ExitoContext);
  const [error, setError] = useState(null);
  const [formData, setFormData] = useState({
    text: '',
  });

  const handleChange = (event) => {
    const { name, value } = event.target;
    setFormData({
      ...formData,
      [name]: value,
    });
  };
  const handleSubmit = async (event) => {
    event.preventDefault();
    if (formData.text === '39785') {
      setExito(true);
      setError(null)
    } else {
      setError('Codigo incorrecto piltrafilla!😈');
    }
  };
  return (
    <div
     
      id="inicio"
      className={`form ${exito === true ? 'hide' : 'active'} form-container`}
    >
      <form action="POST" onSubmit={handleSubmit}>
        <label htmlFor="input-text" id="code">
          Código
        </label>
        <div>
        <input
          type="text"
          placeholder="SECRET CODE"
          id="input-text"
          name="text"
          value={formData.text}
          onChange={handleChange}
          style={{
            border: '1px solid transparent',
            borderColor: error ? 'red' : 'transparent',
          }}
        /> <button type="submit" className="btn input-submit">
        <svg fill='#2661AC' xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M438.6 278.6c12.5-12.5 12.5-32.8 0-45.3l-160-160c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3L338.8 224 32 224c-17.7 0-32 14.3-32 32s14.3 32 32 32l306.7 0L233.4 393.4c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0l160-160z"/></svg>
        </button></div>
        
          <button
            type="button"
            onClick={() =>
              setFormData({
                text: '',
              })
            }
            className="btn delete"
          >
            Borrar
          </button>
        
        {error && <span style={{ color: 'red' }}>{error} </span>}
      </form>
    </div>
  );
}
function SvgForm() {
  const [error, setError] = useState(null);

  const handleSubmit = async (event) => {
    event.preventDefault();
    const formData = new FormData();
    const file = event.target.fileInput.files[0];
    formData.append('svg', file);
    if (file && file.type === 'image/svg+xml') {
      console.log('archivo SVG!', file);

      try {
        const response = await fetch('URL', {
          method: 'POST',

          body: formData,
        });

        if (response.ok) {
          // Manejar la respuesta exitosa aquí
          console.log('Enviado con éxito, Dibujando');
        } else {
          // Manejar errores de la solicitud
          console.error('Error al enviar el formulario');
          setError('Error de solicitud');
        }
      } catch (error) {
        // Manejar errores de red u otros errores
        console.error('Error:', error);
        setError('Error de red u otro');
      }
    } else {
      setError('Por favor, seleccione un archivo SVG.');
    }
  };
  return (
    <form onSubmit={handleSubmit}>
 
      <div className='input-div'>
        <input type="file" name="fileInput" accept="image/svg+xml" className='input-svg' />
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="1.5em"
          height="1.5em"
          strokeLinejoin="round"
          strokeLinecap="round"
          viewBox="0 0 24 24"
          strokeWidth="2"
          fill="none"
          stroke="currentColor"
          className="icon"
        >
          <polyline points="16 16 12 12 8 16"></polyline>
          <line y2="21" x2="12" y1="12" x1="12"></line>
          <path d="M20.39 18.39A5 5 0 0 0 18 9h-1.26A8 8 0 1 0 3 16.3"></path>
          <polyline points="16 16 12 12 8 16"></polyline>
        </svg>
      </div>

      <button type="submit" className="btn input-submit">Enviar</button>
      {error && <span style={{ color: 'red' }}>{error} </span>}
    </form>
  );
}
function TextForm() {
  const [error, setError] = useState(null);
  const handleSubmit = async (event) => {
    event.preventDefault();
    const text = event.target.inputText.value;
    if (text === '') {
      setError('No hay nada que enviar');
    } else {
      console.log('hay texto', text);
      try {
        const response = await fetch('URL', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({ WriteText: text }),
        });

        if (response.ok) {
          // Manejar la respuesta exitosa aquí
          console.log('Enviado con éxito, Dibujando');
        } else {
          // Manejar errores de la solicitud
          console.error('Error al enviar el formulario');
          setError('Error de solicitud');
        }
      } catch (error) {
        // Manejar errores de red u otros errores
        console.error('Error:', error);
        setError('Error de red u otro');
      }
    }
  };
  return (
    <form onSubmit={handleSubmit}>
     
      <label htmlFor="inputText">Escribe una palabra:</label>
      <input
        type="text"
        name="inputText"
        id="inputText"
        style={{
          border: '1px solid transparent',
          borderColor: error ? 'red' : 'transparent',
        }}
      />
      <button type="submit" className="btn input-submit">Enviar</button>
      {error && <span style={{ color: 'red' }}>{error} </span>}
    </form>
  );
}
function FormTwo() {
  const [exito, setExito] = useContext(ExitoContext);
  const handleBack = () => {
    setExito(false);
    console.log(exito);
  };

  return (
    <>
      <div
        id="container-form"
        className={`form ${exito === false ? 'hide' : 'active'}`}
      >
        <SvgForm />
        <TextForm />
        <button type="button" className='back' onClick={handleBack}>
        <svg fill="#E24D57" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M9.4 233.4c-12.5 12.5-12.5 32.8 0 45.3l160 160c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3L109.2 288 416 288c17.7 0 32-14.3 32-32s-14.3-32-32-32l-306.7 0L214.6 118.6c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0l-160 160z"/></svg>
        </button>
      
      </div>
    </>
  );
}
function App() {
  return (
    <>
      <header>
        <motion.div
          id="ibb"
          initial={{ scale: 0 }}
          animate={{ scale: 1 }}
          transition={{
            duration: 1,
            ease: 'easeInOut',
            delay: 0.2,
            type: 'spring',
          }}
        >
       
        </motion.div>
      </header>
      <ExitoProvider>
        <main id="main">
          <Form />
          <FormTwo />
        </main>
      </ExitoProvider>
    </>
  );
}

export default App;

