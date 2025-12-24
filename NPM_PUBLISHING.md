# Guía de Publicación en npm

Esta guía te explicará paso a paso cómo publicar LavinHash en npm.

## 📋 Prerequisitos

1. **Cuenta de npm**: Necesitas una cuenta en [npmjs.com](https://www.npmjs.com/signup)
2. **npm CLI**: Viene incluido con Node.js (verifica con `npm --version`)
3. **Código compilado**: El WASM ya está compilado en el directorio `pkg/`

## 🚀 Pasos para Publicar

### 1. Crear Cuenta en npm (si no tienes una)

```bash
# Abre tu navegador y ve a:
https://www.npmjs.com/signup

# Completa el registro:
- Elige un nombre de usuario
- Correo electrónico
- Contraseña
- Verifica tu correo
```

### 2. Iniciar Sesión en npm desde la Terminal

```bash
npm login

# Te pedirá:
# - Username: tu_usuario_npm
# - Password: tu_contraseña
# - Email: tu_email@ejemplo.com
# - Enter one-time password: (si tienes 2FA activado)
```

Para verificar que estás logueado:
```bash
npm whoami
# Debe mostrar tu nombre de usuario
```

### 3. Verificar el Paquete Antes de Publicar

```bash
cd "C:\Users\Rafa\Desktop\BDOvenbird\Library\LavinHash\pkg"

# Ver qué archivos se incluirán en la publicación
npm pack --dry-run

# Esto mostrará:
# - Tamaño del paquete
# - Lista de archivos que se incluirán
# - NO crea ningún archivo, solo muestra la información
```

### 4. Probar el Paquete Localmente (Opcional pero Recomendado)

```bash
# Crear un tarball del paquete
npm pack

# Esto crea: lavinhash-1.0.0.tgz
# Puedes instalarlo en otro proyecto para probarlo:
# npm install /ruta/a/lavinhash-1.0.0.tgz
```

### 5. Publicar en npm

```bash
cd "C:\Users\Rafa\Desktop\BDOvenbird\Library\LavinHash\pkg"

# Publicar el paquete
npm publish

# Si es la primera vez, npm publicará la versión 1.0.0
# Si todo va bien, verás:
# + lavinhash@1.0.0
```

### 6. Verificar la Publicación

```bash
# Ver tu paquete en npm
npm view lavinhash

# O visita:
https://www.npmjs.com/package/lavinhash
```

## 🔄 Actualizar una Versión Publicada

Si necesitas publicar una actualización:

```bash
# 1. Actualiza la versión en Cargo.toml
# version = "1.0.1"  (patch)
# version = "1.1.0"  (minor)
# version = "2.0.0"  (major)

# 2. Recompila el WASM
cd "C:\Users\Rafa\Desktop\BDOvenbird\Library\LavinHash"
wasm-pack build --target web --out-dir pkg --out-name lavinhash

# 3. Publica la nueva versión
cd pkg
npm publish
```

## 📦 Comandos Útiles

```bash
# Ver información de tu paquete
npm view lavinhash

# Ver todas las versiones publicadas
npm view lavinhash versions

# Despublicar una versión (solo en las primeras 72 horas)
npm unpublish lavinhash@1.0.0

# Deprecar una versión (preferido sobre unpublish)
npm deprecate lavinhash@1.0.0 "Esta versión tiene un bug, usar 1.0.1"

# Ver estadísticas de descargas
npm view lavinhash downloads

# Cerrar sesión
npm logout
```

## 🛡️ Buenas Prácticas

### 1. Versionado Semántico (SemVer)

- **MAJOR** (1.x.x → 2.x.x): Cambios incompatibles con versiones anteriores
- **MINOR** (x.1.x → x.2.x): Nuevas funcionalidades compatibles
- **PATCH** (x.x.1 → x.x.2): Correcciones de bugs

### 2. Antes de Publicar

✅ Verifica que el código compila: `cargo test`
✅ Compila el WASM: `wasm-pack build`
✅ Revisa el package.json
✅ Actualiza README.md si es necesario
✅ Actualiza CHANGELOG.md

### 3. Después de Publicar

✅ Verifica la página de npm
✅ Prueba instalar tu paquete: `npm install lavinhash`
✅ Actualiza la documentación si cambió algo
✅ Crea un git tag: `git tag v1.0.0 && git push origin v1.0.0`

## 🔒 Seguridad

### Habilitar 2FA (Autenticación de Dos Factores)

Es **altamente recomendado** para proteger tu cuenta:

```bash
# 1. Ve a tu perfil en npmjs.com
# 2. Settings → Two-Factor Authentication
# 3. Sigue las instrucciones para configurar 2FA
```

### Tokens de Acceso

Para CI/CD, usa tokens en lugar de tu contraseña:

```bash
# 1. Ve a npmjs.com → Settings → Access Tokens
# 2. Generate New Token → Automation
# 3. Copia el token y úsalo en tu CI/CD
```

## ❌ Errores Comunes

### "You do not have permission to publish"

**Solución**: El nombre `lavinhash` ya está tomado o no tienes permisos.
```bash
# Verifica si el nombre está disponible
npm view lavinhash

# Si está tomado, cambia el nombre en package.json a:
# "@tu-usuario/lavinhash"
```

### "You must be logged in"

**Solución**: Inicia sesión
```bash
npm login
```

### "This package has been marked as private"

**Solución**: Verifica que package.json NO tenga `"private": true`

### "Package size exceeds 10MB"

**Solución**: El WASM es demasiado grande. Intenta:
```bash
# Reducir el tamaño con optimizaciones
# O excluir archivos innecesarios en .npmignore
```

## 📊 Monitorear tu Paquete

### npm Stats

```bash
# Ver descargas de la última semana
npm view lavinhash downloads

# Sitios web para estadísticas:
https://npmtrends.com/lavinhash
https://npm-stat.com/charts.html?package=lavinhash
```

### Actualizaciones de Dependencias

```bash
# Usa dependabot en GitHub para mantener dependencias actualizadas
# O revisa manualmente con:
npm outdated
```

## 🎯 Resumen de Comandos

```bash
# PRIMERA VEZ:
npm login                              # Iniciar sesión
cd pkg                                 # Ir al directorio del paquete
npm publish                            # Publicar

# ACTUALIZACIONES:
# 1. Actualizar version en Cargo.toml
wasm-pack build --target web --out-dir pkg --out-name lavinhash
cd pkg
npm publish

# VERIFICACIÓN:
npm view lavinhash                     # Ver info del paquete
npm install lavinhash                  # Probar instalación
```

## 🆘 Ayuda

- **Documentación oficial**: https://docs.npmjs.com/
- **Soporte npm**: https://npm.community/
- **Guía de publicación**: https://docs.npmjs.com/packages-and-modules/contributing-packages-to-the-registry

---

**¡Listo!** Una vez publicado, cualquiera podrá instalar tu paquete con:

```bash
npm install lavinhash
```

o

```bash
yarn add lavinhash
```
