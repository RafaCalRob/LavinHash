# 🚀 LavinHash - Guía Rápida

¡Todo está listo para publicar! Aquí tienes los pasos finales.

## ✅ Lo que ya está hecho

- ✅ Código Rust compilado y probado
- ✅ WASM compilado exitosamente (directorio `pkg/`)
- ✅ package.json configurado para npm
- ✅ Documentación completa
- ✅ Ejemplos de uso (Rust, JavaScript, Browser)
- ✅ Git inicializado con commits
- ✅ Licencia MIT
- ✅ README profesional

## 📦 Publicar en npm (3 pasos simples)

### 1️⃣ Crear cuenta en npm (si no tienes)

Ve a: https://www.npmjs.com/signup

### 2️⃣ Iniciar sesión desde terminal

```bash
npm login
# Te pedirá: usuario, contraseña, email
```

### 3️⃣ Publicar

```bash
cd "C:\Users\Rafa\Desktop\BDOvenbird\Library\LavinHash\pkg"
npm publish
```

**¡Eso es todo!** En unos segundos estará en npm.

## 🔍 Verificar publicación

```bash
npm view lavinhash
```

O visita: https://www.npmjs.com/package/lavinhash

## 🌐 Subir a GitHub

```bash
cd "C:\Users\Rafa\Desktop\BDOvenbird\Library\LavinHash"

git branch -M main
git remote add origin https://github.com/RafaCalRob/LavinHash.git
git push -u origin main
```

## 📝 Probar la librería

### En Node.js:

```bash
npm install lavinhash
```

```javascript
import init, { wasm_compare_data } from 'lavinhash';

await init();

const encoder = new TextEncoder();
const text1 = encoder.encode("Hello World");
const text2 = encoder.encode("Hello World!");

const similarity = wasm_compare_data(text1, text2);
console.log('Similarity:', similarity + '%');
```

### En el navegador:

```html
<script type="module">
  import init, { wasm_compare_data } from './pkg/lavinhash.js';
  await init();
  // ... usar wasm_compare_data
</script>
```

## 📚 Archivos importantes

- **NPM_PUBLISHING.md** → Guía completa de publicación en npm
- **README.md** → Documentación principal
- **docs/TECHNICAL.md** → Especificación técnica del algoritmo
- **examples/** → Ejemplos de uso en Rust, JS y Browser
- **pkg/** → Paquete compilado listo para npm

## 🔄 Actualizar versión

```bash
# 1. Edita Cargo.toml y cambia: version = "1.0.1"

# 2. Recompila WASM
wasm-pack build --target web --out-dir pkg --out-name lavinhash

# 3. Publica
cd pkg
npm publish
```

## 📊 Estadísticas del paquete

Una vez publicado:
- **Tamaño**: ~180KB (WASM incluido)
- **Funciones exportadas**: 4 (generate_hash, compare_hashes, compare_data, fingerprint_size)
- **Plataformas**: Node.js 14+, navegadores modernos

## ❓ Preguntas Frecuentes

**P: ¿El nombre "lavinhash" está disponible?**
R: Verifica con `npm view lavinhash`. Si está tomado, usa `@tu-usuario/lavinhash`

**P: ¿Cuánto cuesta publicar en npm?**
R: Es completamente gratis

**P: ¿Puedo despublicar si me equivoco?**
R: Sí, en las primeras 72 horas con `npm unpublish lavinhash@1.0.0`

**P: ¿Necesito recompilar para cada actualización?**
R: Sí, pero es rápido: `wasm-pack build --target web --out-dir pkg --out-name lavinhash`

## 🆘 Ayuda

Si tienes problemas:
1. Lee **NPM_PUBLISHING.md** (guía detallada con soluciones)
2. Verifica que estás en el directorio `pkg/` al publicar
3. Asegúrate de estar logueado: `npm whoami`

## 🎉 ¡Listo!

Tu librería está lista para el mundo. Solo faltan 3 comandos:

```bash
npm login
cd pkg
npm publish
```

**Éxito garantizado** 🚀
