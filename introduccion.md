[TOC]
# Introducción
## ¿Que es?
- Es un lenguaje de programación compilado de propósito general.
## Propiedades
- Se compila en código nativo, no se requiere tener instalado el lenguaje rust para ejecutar el código.
- Apareció en el 2010.
- Inicio su desarrollo con Graydon Hoare y Mozilla.
- Se enfoca en la seguridad, velocidad y concurrencia.
- Tiene abstracciones de costo cero.
- Los archivos tiene la extensión `.rs`.
- Utiliza la licencia Apache 2.0 | MIT.
- Maneja un sistema de tipos.
## Ventajas
- Es tan rápido como C o C++.
- Es eficiente en memoria, porque no tiene un recolector de basura. Para esto agregar código de liberación de memoria en tiempo de compilación según el tiempo de vida de la variable, su ámbito o referencias de la variable.
- Permite programar con mayor velocidad, confianza y seguridad el nivel inferior, bajo o sistemas sin la preocupación de que existan agujeros de seguridad, vulnerabilidades, errores o fallos (explotaciones, bloqueos o corrupción).
- Permite reducir los riesgos al ejecutar operaciones paralelizadas.
- El compilador detectará errores comunes y no compila el código. Por lo que reduce la necesidad de realizar pruebas exhaustivas (revisión cuidadosa del código por parte de desarrolladores experimentados).
- El compilador optimiza el código.
- La base del lenguaje está en el compilador, el cual se encarga de agregar casi todas las ventajas anteriormente mencionadas.
- Sintácticamente ofrece ergonomía al tratar de unificar el alto nivel (CLI, API) y el control del bajo nivel (Administración de memoria, concurrencia).
- Utiliza un tipo estático (conoce todas las variables en tiempo de compilación).
## Desventajas
- La curva de aprendizaje es alta.
## Herramientas
### Cargo
- Es el administrador de dependencias
- Es la herramienta de construcción de dependencias y programa final.
### RustUp
- Es el  administrador de versiones del lenguaje rust.