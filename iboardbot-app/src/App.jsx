import { useContext, useState, createContext } from "react";
import { motion } from "framer-motion";

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
    text: "",
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
    if (formData.text === "39785") {
      setExito(true);
      setError(null);
    } else {
      setError("Codigo incorrecto piltrafilla!😈");
    }
  };
  return (
    <div
      id="inicio"
      className={`form ${exito === true ? "hide" : "active"} form-container`}
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
              border: "1px solid transparent",
              borderColor: error ? "red" : "transparent",
            }}
          />{" "}
          <button type="submit" className="btn input-submit">
            <svg
              fill="#2661AC"
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 448 512"
            >
              <path d="M438.6 278.6c12.5-12.5 12.5-32.8 0-45.3l-160-160c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3L338.8 224 32 224c-17.7 0-32 14.3-32 32s14.3 32 32 32l306.7 0L233.4 393.4c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0l160-160z" />
            </svg>
          </button>
        </div>

        <button
          type="button"
          onClick={() =>
            setFormData({
              text: "",
            })
          }
          className="btn delete"
        >
          Borrar
        </button>

        {error && <span style={{ color: "red" }}>{error} </span>}
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
    formData.append("svg", file);
    if (file && file.type === "image/svg+xml") {
      console.log("archivo SVG!", file);

      try {
        const response = await fetch(
          "http://ibb.muevetef/api/boards/jobs/main/file",
          {
            method: "POST",

            body: formData,
          }
        );

        if (response.ok) {
          // Manejar la respuesta exitosa aquí
          console.log("Enviado con éxito, Dibujando");
        } else {
          // Manejar errores de la solicitud
          console.error("Error al enviar el formulario");
          setError("Error de solicitud");
        }
      } catch (error) {
        // Manejar errores de red u otros errores
        console.error("Error:", error);
        setError("Error de red u otro");
      }
    } else {
      setError("Por favor, seleccione un archivo SVG.");
    }
  };
  return (
    <form onSubmit={handleSubmit}>
      <div className="input-div">
        <input
          type="file"
          name="fileInput"
          accept="image/svg+xml"
          className="input-svg"
        />
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

      <button type="submit" className="btn input-submit">
        Enviar
      </button>
      {error && <span style={{ color: "red" }}>{error} </span>}
    </form>
  );
}
function TextForm() {
  const [error, setError] = useState(null);
  const handleSubmit = async (event) => {
    event.preventDefault();
    const text = event.target.inputText.value;
    if (text === "") {
      setError("No hay nada que enviar");
    } else {
      console.log("hay texto", text);
      try {
        const response = await fetch(
          "http://ibb.muevetef/api/boards/jobs/main",
          {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify({ WriteText: text }),
          }
        );

        if (response.ok) {
          // Manejar la respuesta exitosa aquí
          console.log("Enviado con éxito, Dibujando");
        } else {
          // Manejar errores de la solicitud
          console.error("Error al enviar el formulario");
          setError("Error de solicitud");
        }
      } catch (error) {
        // Manejar errores de red u otros errores
        console.error("Error de red u otro", error);
        setError("Error, vuelve a intentarlo");
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
          border: "1px solid transparent",
          borderColor: error ? "red" : "transparent",
        }}
      />
      <button type="submit" className="btn input-submit">
        Pintar
      </button>
      {error && <span style={{ color: "red" }}>{error} </span>}
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
        className={`form ${exito === false ? "hide" : "active"}`}
      >
        <SvgForm />
        <TextForm />
        <button type="button" className="back" onClick={handleBack}>
          <svg
            fill="#E24D57"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 448 512"
          >
            <path d="M9.4 233.4c-12.5 12.5-12.5 32.8 0 45.3l160 160c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3L109.2 288 416 288c17.7 0 32-14.3 32-32s-14.3-32-32-32l-306.7 0L214.6 118.6c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0l-160 160z" />
          </svg>
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
            duration: 0.5,
            ease: "easeInOut",
            delay: 0.2,
            type: "spring",
          }}
        >
          <svg
            width="197"
            height="69"
            viewBox="0 0 197 69"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              d="M11.543 23.0682L9.39453 49.6698L1.15234 50.0995V23.6151L11.543 23.0682Z"
              fill="#E24D57"
            />
            <path
              d="M21.2285 19.6584C17.7036 20.3382 15.3972 23.7468 16.077 27.2717L22.6414 61.3093C23.3212 64.8342 26.7298 67.1406 30.2547 66.4608L119.359 49.2764C122.883 48.5966 125.19 45.188 124.51 41.6631L117.946 7.62548C117.266 4.10058 113.857 1.79418 110.332 2.47399L21.2285 19.6584Z"
              fill="white"
              stroke="#285FB0"
              strokeWidth="3"
            />
            <path
              d="M46.4531 39.2885L46.5389 40.8466L44.3524 41.1035C44.1837 42.9898 43.7867 44.5335 43.1613 45.7344C42.5626 46.9331 41.4929 48.2635 39.9522 49.7256C38.4116 51.1877 36.5204 52.4507 34.2788 53.5146C32.0659 54.603 30.1886 55.2094 28.647 55.3339C27.1053 55.4584 25.91 55.2339 25.0611 54.6603C24.2143 54.1134 23.7502 53.3349 23.6686 52.3249C23.5441 50.7832 24.3233 49.1686 26.0063 47.481C27.7136 45.7646 29.8146 44.284 32.3092 43.0392C34.8282 41.7656 37.3485 40.8397 39.8701 40.2615C39.4177 39.6292 38.7624 39.1337 37.9041 38.775C37.0723 38.4141 35.6648 38.0462 33.6815 37.6713C32.5947 37.4648 32.0245 37.0293 31.9708 36.3648C31.9386 35.9661 32.0691 35.5944 32.3623 35.2496C32.6799 34.8762 33.2059 34.4324 33.9403 33.9183C34.699 33.3755 35.4046 32.8369 36.0571 32.3026C36.734 31.7397 37.4698 31.0783 38.2645 30.3185C39.0859 29.5566 39.7588 28.7799 40.2835 27.9884C40.8082 27.197 41.048 26.5221 41.0029 25.964C40.9277 25.0337 40.425 24.6061 39.4947 24.6812C38.7771 24.7392 37.8215 25.1641 36.628 25.9561C35.459 26.7193 34.3194 27.6809 33.2092 28.8407L31.9326 34.4015C31.0435 38.299 29.9528 41.3567 28.6604 43.5746C27.3947 45.7904 26.1106 46.9508 24.8082 47.056C23.426 47.1677 22.6662 46.3729 22.5288 44.6718C22.3915 42.9706 23.0405 40.7378 24.476 37.9733C25.9093 35.1822 27.9751 32.2731 30.6736 29.2461C30.6714 29.2195 30.7185 28.9749 30.815 28.5123C30.9358 28.021 31.0433 27.5308 31.1376 27.0416C31.4736 25.5698 31.745 24.6248 31.9521 24.2068L33.9315 24.368C33.9722 24.873 33.9142 25.4797 33.7574 26.1879C35.1557 24.9513 36.6456 23.8545 38.227 22.8975C39.8329 21.9117 41.0212 21.3876 41.792 21.3254C42.5894 21.261 43.2478 21.462 43.767 21.9284C44.2841 22.3682 44.5813 23.0665 44.6585 24.0234C44.7337 24.9537 44.3687 26.0667 43.5638 27.3624C42.7854 28.6559 41.7714 29.848 40.5218 30.9388C38.249 32.9416 36.2942 34.4104 34.6574 35.3452C37.4561 35.5472 39.5224 35.9556 40.8562 36.5702C42.1878 37.1582 43.2029 38.133 43.9015 39.4945L46.4531 39.2885ZM27.1503 41.7703C28.7683 38.6165 29.7553 35.4334 30.1112 32.221C28.8025 33.9052 27.645 35.8045 26.6386 37.919C25.6322 40.0336 25.1655 41.5427 25.2385 42.4464C25.2836 43.0046 25.4922 43.2687 25.8643 43.2386C26.263 43.2064 26.6917 42.717 27.1503 41.7703ZM37.0892 49.5556C38.3208 48.5732 39.278 47.506 39.9608 46.354C40.6436 45.202 40.9464 44.1475 40.8691 43.1906C40.8326 42.7388 40.7297 42.2923 40.5603 41.8511C36.6119 42.6516 33.4141 43.9666 30.9667 45.7962C28.5173 47.5992 27.3516 49.2317 27.4696 50.6936C27.5211 51.3315 27.8144 51.8161 28.3495 52.1475C28.8867 52.5054 29.5939 52.6489 30.4711 52.5781C32.624 52.4042 34.8301 51.3967 37.0892 49.5556ZM59.0371 33.6171C59.8205 34.7042 60.268 35.9389 60.3797 37.321C60.5535 39.474 59.9649 41.4612 58.6138 43.2825C57.2606 45.0773 55.5739 46.0562 53.5538 46.2194C52.0121 46.3439 50.7673 46.003 49.8193 45.1967C48.8958 44.3616 48.3707 43.16 48.2441 41.5918C48.0401 39.0667 48.8411 36.7279 50.6471 34.5756C52.4509 32.3966 54.5754 31.2084 57.0208 31.0109C57.4727 30.9744 57.7927 30.962 57.9809 30.9735L58.2068 32.2796C56.0804 32.4513 54.3622 33.2054 53.0522 34.5419C51.7422 35.8783 51.1644 37.5034 51.319 39.4172C51.4113 40.5601 51.7655 41.4679 52.3817 42.1405C52.9956 42.7865 53.8342 43.0666 54.8975 42.9807C55.9872 42.8927 56.8125 42.3445 57.3731 41.3361C57.9337 40.3277 58.1625 39.1856 58.0595 37.9097C57.9543 36.6073 57.6063 35.4449 57.0154 34.4224L59.0371 33.6171ZM78.6088 39.902L79.5805 40.5058C79.2728 41.3332 78.6429 42.1466 77.6908 42.9458C76.7653 43.7429 75.8905 44.1747 75.0666 44.2412C73.4983 44.3679 72.6477 43.6072 72.5146 41.9593C72.4523 41.1884 72.5776 40.2553 72.8905 39.1599L71.5376 40.7941C70.2204 42.372 69.0505 43.4563 68.0281 44.0472C67.0323 44.6359 66.0293 44.9711 65.0193 45.0527C64.0092 45.1342 63.2159 44.9174 62.6392 44.4021C62.0869 43.8582 61.77 43.0811 61.6885 42.0711C61.5382 40.2105 62.1077 38.3185 63.3971 36.3951C64.6843 34.4452 66.3981 32.8086 68.5386 31.4853C70.679 30.1621 72.8645 29.397 75.0951 29.1901L75.2811 30.4994L73.6355 30.9935C71.2596 31.7204 69.2058 32.9564 67.474 34.7015C65.7688 36.4444 64.9805 38.1133 65.1093 39.7081C65.236 41.2763 65.9372 42.0089 67.2131 41.9058C68.4092 41.8093 69.8168 40.8528 71.4358 39.0366C73.0549 37.2204 74.3579 35.1354 75.345 32.7816L77.1 33.6432C75.7204 37.0988 75.0671 39.2784 75.1401 40.1822C75.1701 40.5543 75.3155 40.8636 75.5762 41.1101C75.8613 41.3278 76.1235 41.427 76.3627 41.4077C76.6285 41.3863 76.8002 41.359 76.8778 41.326C76.9554 41.293 77.033 41.2599 77.1105 41.2269C77.186 41.1673 77.2737 41.0934 77.3735 41.005C77.5 40.9146 77.5999 40.8262 77.6732 40.7401C77.7731 40.6517 77.9229 40.5192 78.1226 40.3426C78.3224 40.1659 78.4844 40.0191 78.6088 39.902ZM90.5426 28.7853C91.2071 28.7316 91.7703 28.9136 92.232 29.3311C92.6938 29.7486 92.9537 30.3162 93.0116 31.0339C93.1276 32.4692 92.4259 34.0508 90.9068 35.7787L89.8887 35.0984C90.1943 34.2444 90.332 33.6313 90.302 33.2592C90.244 32.5415 89.8163 32.2149 89.0189 32.2793C88.2215 32.3437 87.5048 32.9099 86.8687 33.9779C86.2326 35.0459 85.5087 36.6828 84.6969 38.8887C83.9096 41.0658 83.2989 42.6133 82.8647 43.5312L80.5116 42.718C81.7052 39.2774 82.3847 37.2562 82.5501 36.6542C83.1003 34.523 83.2467 31.8626 82.9891 28.673L85.1735 27.8946C85.2333 29.6288 85.2221 30.9807 85.1399 31.9505C85.0555 32.8937 84.7935 34.4532 84.3539 36.6289C86.6457 31.548 88.7086 28.9334 90.5426 28.7853ZM114.799 11.3763L116.89 11.9297L116.416 13.0114C113.932 18.6965 111.873 24.1733 110.238 29.442C109.153 32.9006 108.678 35.4671 108.813 37.1417C108.848 37.567 108.982 37.9039 109.216 38.1526C109.448 38.3746 109.79 38.4674 110.242 38.4309C110.72 38.3923 111.24 38.0426 111.802 37.3819L112.575 38.0017C112.291 38.8004 111.751 39.5664 110.954 40.2996C110.155 41.0062 109.357 41.3917 108.56 41.4561C107.018 41.5806 106.174 40.7391 106.028 38.9317C105.943 37.8685 106.103 36.2102 106.509 33.9568C103.815 39.6856 100.966 42.6713 97.9628 42.9139C96.9261 42.9976 96.0764 42.7452 95.4135 42.1566C94.7507 41.5681 94.3752 40.7289 94.2872 39.6391C94.1477 37.9114 94.6939 36.0614 95.9258 34.0892C97.1577 32.117 98.7365 30.4645 100.662 29.1318C102.586 27.7726 104.424 27.0221 106.179 26.8805L106.411 28.2663C104.149 28.7433 102.09 29.9128 100.233 31.7749C98.4018 33.6083 97.5568 35.4022 97.6984 37.1565C97.7586 37.9007 98.0062 38.4827 98.4414 38.9023C98.8744 39.2954 99.4099 39.4662 100.048 39.4147C101.35 39.3095 102.939 37.7766 104.813 34.8161C105.502 33.7439 106.26 32.1978 107.087 30.178C107.941 28.1561 108.612 26.5368 109.103 25.32C109.617 24.0746 110.109 22.8711 110.577 21.7097C111.069 20.5195 111.707 18.9698 112.489 17.0607C113.269 15.1249 114.039 13.2301 114.799 11.3763Z"
              fill="#153355"
            />
            <path
              d="M149.73 43.6151C149.73 44.6958 149.529 45.6268 149.125 46.4081C148.734 47.1893 148.207 47.8534 147.543 48.4002C146.879 48.9471 146.111 49.3963 145.238 49.7479C144.379 50.0864 143.48 50.3534 142.543 50.5487C141.605 50.744 140.661 50.8742 139.711 50.9393C138.773 51.0044 137.895 51.037 137.074 51.037C136.267 51.037 135.427 51.0044 134.555 50.9393C133.695 50.8872 132.836 50.7896 131.977 50.6463C131.117 50.5031 130.271 50.3143 129.438 50.0799C128.604 49.8325 127.816 49.5265 127.074 49.162L127.035 24.0057C127.764 23.7062 128.539 23.4393 129.359 23.2049C130.18 22.9575 131.013 22.7557 131.859 22.5995C132.719 22.4302 133.572 22.3065 134.418 22.2284C135.264 22.1372 136.085 22.0916 136.879 22.0916C137.803 22.0916 138.734 22.1502 139.672 22.2674C140.622 22.3846 141.534 22.5799 142.406 22.8534C143.279 23.1268 144.092 23.4849 144.848 23.9276C145.603 24.3573 146.26 24.8911 146.82 25.5291C147.393 26.1672 147.836 26.9094 148.148 27.7557C148.474 28.6021 148.637 29.5786 148.637 30.6854C148.637 31.4146 148.533 32.0982 148.324 32.7362C148.116 33.3742 147.816 33.9471 147.426 34.4549C147.035 34.9627 146.56 35.3989 146 35.7635C145.44 36.1151 144.809 36.3755 144.105 36.5448C144.952 36.7661 145.72 37.0916 146.41 37.5213C147.113 37.951 147.706 38.4719 148.188 39.0838C148.682 39.6958 149.06 40.3859 149.32 41.1541C149.594 41.9224 149.73 42.7427 149.73 43.6151ZM139.809 32.4823C139.809 32.1047 139.73 31.7857 139.574 31.5252C139.418 31.2518 139.223 31.0304 138.988 30.8612C138.767 30.6919 138.52 30.5682 138.246 30.4901C137.986 30.412 137.751 30.3729 137.543 30.3729C137.283 30.3729 137.029 30.412 136.781 30.4901C136.534 30.5682 136.293 30.6594 136.059 30.7635L135.941 35.0995C136.241 35.0995 136.612 35.0734 137.055 35.0213C137.497 34.9692 137.921 34.8586 138.324 34.6893C138.741 34.507 139.092 34.2466 139.379 33.9081C139.665 33.5565 139.809 33.0812 139.809 32.4823ZM137.035 38.8495C136.658 38.8495 136.293 38.8885 135.941 38.9666L135.863 43.4198C136.059 43.4588 136.247 43.4979 136.43 43.537C136.625 43.563 136.814 43.576 136.996 43.576C137.27 43.576 137.569 43.55 137.895 43.4979C138.22 43.4328 138.526 43.3156 138.812 43.1463C139.099 42.964 139.333 42.7232 139.516 42.4237C139.711 42.1242 139.809 41.7271 139.809 41.2323C139.809 40.7375 139.711 40.3338 139.516 40.0213C139.333 39.7088 139.105 39.4679 138.832 39.2987C138.559 39.1164 138.259 38.9992 137.934 38.9471C137.608 38.882 137.309 38.8495 137.035 38.8495ZM174.457 35.6073C174.457 36.7922 174.32 37.938 174.047 39.0448C173.773 40.1385 173.376 41.1672 172.855 42.1307C172.348 43.0942 171.723 43.9797 170.98 44.787C170.251 45.5812 169.424 46.2648 168.5 46.8377C167.589 47.4107 166.592 47.8599 165.512 48.1854C164.431 48.4979 163.285 48.6541 162.074 48.6541C160.902 48.6541 159.783 48.5044 158.715 48.2049C157.66 47.9054 156.671 47.4888 155.746 46.9549C154.822 46.4081 153.982 45.757 153.227 45.0018C152.484 44.2336 151.846 43.3872 151.312 42.4627C150.792 41.5252 150.382 40.5226 150.082 39.4549C149.796 38.3872 149.652 37.2739 149.652 36.1151C149.652 34.9823 149.789 33.8755 150.062 32.7948C150.336 31.701 150.727 30.6724 151.234 29.7088C151.755 28.7453 152.38 27.8599 153.109 27.0526C153.839 26.2453 154.652 25.5487 155.551 24.9627C156.462 24.3768 157.445 23.9211 158.5 23.5956C159.555 23.27 160.668 23.1073 161.84 23.1073C163.728 23.1073 165.447 23.4002 166.996 23.9862C168.559 24.5721 169.887 25.412 170.98 26.5057C172.087 27.5864 172.94 28.9015 173.539 30.451C174.151 31.9875 174.457 33.7062 174.457 35.6073ZM165.785 36.1151C165.785 35.5552 165.701 35.0148 165.531 34.494C165.375 33.9601 165.141 33.4914 164.828 33.0877C164.516 32.6711 164.125 32.339 163.656 32.0916C163.201 31.8312 162.673 31.701 162.074 31.701C161.462 31.701 160.915 31.8117 160.434 32.0331C159.952 32.2544 159.535 32.5604 159.184 32.951C158.845 33.3286 158.585 33.7778 158.402 34.2987C158.22 34.8065 158.129 35.3469 158.129 35.9198C158.129 36.4666 158.207 37.0135 158.363 37.5604C158.52 38.1073 158.754 38.6021 159.066 39.0448C159.379 39.4875 159.763 39.8456 160.219 40.119C160.688 40.3924 161.228 40.5291 161.84 40.5291C162.452 40.5291 162.999 40.412 163.48 40.1776C163.975 39.9302 164.392 39.6047 164.73 39.201C165.069 38.7844 165.329 38.3091 165.512 37.7752C165.694 37.2414 165.785 36.688 165.785 36.1151ZM196.508 23.0291L196.312 31.3104L190.492 31.5448L189.789 49.6698L181.547 50.0995L180.805 31.9745L174.984 32.287L175.219 23.0682L196.508 23.0291Z"
              fill="#E8505C"
            />
          </svg>
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
