﻿//! 为一些基本类型和常用类型提供预定义布局。

#![allow(missing_docs)]

layout!(U8        u(  8));
layout!(U16       u( 16));
layout!(U32       u( 32));
layout!(U64       u( 64));
layout!(U128      u(128));
layout!(I8        i(  8));
layout!(I16       i( 16));
layout!(I32       i( 32));
layout!(I64       i( 64));
layout!(I128      i(128));
layout!(F16  e( 5)m( 10));
layout!(BF16 e( 8)m(  7));
layout!(F32  e( 8)m( 23));
layout!(F64  e(11)m( 52));
layout!(F128 e(15)m(112));
layout!(Bool; [1] in 1);
