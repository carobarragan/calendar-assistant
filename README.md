// # 📘 Documentación: Asistente Virtual Inteligente

// ## 🎯 Objetivo del Proyecto

// Desarrollar un asistente virtual con capacidades básicas de comprensión de lenguaje natural (tipo Alexa) que interactúe con el usuario a través de texto (y eventualmente voz), con funciones como gestionar calendario, responder preguntas, y expandirse con nuevas funcionalidades.

// ---

// ## 🧱 Arquitectura General

// **Componentes principales:**

// 1. **Frontend (cliente)**
// - Interfaz visual en SvelteKit + Skeleton UI.
// - Permite al usuario escribir (o decir) comandos.
// - Muestra respuestas del asistente.
// - Opcional: soporte de voz (micrófono y parlante).

// 2. **Backend (servidor)**
// - Escrito en Rust.
// - Recibe los mensajes del usuario.
// - Interpreta comandos usando un motor tipo `interpret_command`.
// - Ejecuta acciones (consultar calendario, responder, etc).
// - Devuelve una respuesta estructurada al frontend.

// ---

// ## 📌 Funcionalidades Planeadas

// | Funcionalidad | Estado | Descripción |
// |---------------------------|---------------|---------------------------------------------------------------|
// | Interfaz Web | 🟡 En progreso | Input de texto, salida de texto, botón de enviar |
// | Comando de Calendario | 🔜 Planeado | "¿Qué tengo mañana?" → Lista eventos |
// | IA Respuestas Básicas | 🔜 Planeado | "¿Cómo estás?", "¿Qué podés hacer?" |
// | Motor de Interpretación | 🟡 En progreso | Detectar intención y derivar a servicio adecuado |
// | API REST | ✅ Hecho | Endpoint para recibir y responder a mensajes del usuario |
// | Persistencia de Eventos | 🔜 Planeado | Guardar eventos en base de datos o en memoria |
// | Soporte de Voz | 🔜 Opcional | Voz a texto (input) / Texto a voz (output) |

// ---

// ## 🔁 Flujo de Datos

# calendar-assistant
