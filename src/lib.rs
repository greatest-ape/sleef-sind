#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

mod tables;

use core::arch::x86_64::*;

use tables::*;

type vmask_avx_sleef = __m256i;
type vopmask_avx_sleef = __m256i;

type vdouble_avx_sleef = __m256d;
type vint_avx_sleef = __m128i;

macro_rules! vsll_vi_vi_i_avx_sleef {
    ($x:expr, $c:expr) => {
        _mm_slli_epi32($x, $c)
    };
}

macro_rules! vsra_vi_vi_i_avx_sleef {
    ($x:expr, $c:expr) => {
        _mm_srai_epi32($x, $c)
    };
}

macro_rules! vsrl_vi_vi_i_avx_sleef {
    ($x:expr, $c:expr) => {
        _mm_srli_epi32($x, $c)
    };
}

#[derive(Clone, Copy)]
struct vint2_avx_sleef {
    x: __m128i,
    y: __m128i,
}

#[derive(Clone, Copy)]
struct vdouble2_avx_sleef {
    x: vdouble_avx_sleef,
    y: vdouble_avx_sleef,
}

fn vd2getx_vd_vd2_avx_sleef(v: vdouble2_avx_sleef) -> vdouble_avx_sleef {
    v.x
}
fn vd2gety_vd_vd2_avx_sleef(v: vdouble2_avx_sleef) -> vdouble_avx_sleef {
    v.y
}

unsafe fn vcast_vd2_vd_vd_avx_sleef(
    h: vdouble_avx_sleef,
    l: vdouble_avx_sleef,
) -> vdouble2_avx_sleef {
    vd2setxy_vd2_vd_vd_avx_sleef(h, l)
}

const fn vd2setxy_vd2_vd_vd_avx_sleef(
    x: vdouble_avx_sleef,
    y: vdouble_avx_sleef,
) -> vdouble2_avx_sleef {
    vdouble2_avx_sleef { x, y }
}

#[derive(Clone, Copy)]
struct di_t_avx_sleef {
    d: vdouble_avx_sleef,
    i: vint_avx_sleef,
}

unsafe fn disetdi_di_vd_vi_avx_sleef(d: vdouble_avx_sleef, i: vint_avx_sleef) -> di_t_avx_sleef {
    di_t_avx_sleef { d, i }
}

#[derive(Clone, Copy)]
struct ddi_t_avx_sleef {
    dd_avx_sleef: vdouble2_avx_sleef,
    i: vint_avx_sleef,
}

unsafe fn ddigetdd_vd2_ddi_avx_sleef(d: ddi_t_avx_sleef) -> vdouble2_avx_sleef {
    d.dd_avx_sleef
}

fn ddisetdd_ddi_ddi_vd2_avx_sleef(
    mut ddi_avx_sleef: ddi_t_avx_sleef,
    v: vdouble2_avx_sleef,
) -> ddi_t_avx_sleef {
    ddi_avx_sleef.dd_avx_sleef = v;
    return ddi_avx_sleef;
}

unsafe fn vsel_vd2_vo_vd2_vd2_avx_sleef(
    m: vopmask_avx_sleef,
    x: vdouble2_avx_sleef,
    y: vdouble2_avx_sleef,
) -> vdouble2_avx_sleef {
    return vd2setxy_vd2_vd_vd_avx_sleef(
        vsel_vd_vo_vd_vd_avx_sleef(m, vd2getx_vd_vd2_avx_sleef(x), vd2getx_vd_vd2_avx_sleef(y)),
        vsel_vd_vo_vd_vd_avx_sleef(m, vd2gety_vd_vd2_avx_sleef(x), vd2gety_vd_vd2_avx_sleef(y)),
    );
}

unsafe fn vsel_vd_vo_vd_vd_avx_sleef(
    o: vopmask_avx_sleef,
    x: vdouble_avx_sleef,
    y: vdouble_avx_sleef,
) -> vdouble_avx_sleef {
    return _mm256_blendv_pd(y, x, _mm256_castsi256_pd(o));
}

#[inline]
unsafe fn vtestallones_i_vo64_avx_sleef(g: vopmask_avx_sleef) -> i32 {
    _mm_test_all_ones(_mm_and_si128(
        _mm256_extractf128_si256(g, 0),
        _mm256_extractf128_si256(g, 1),
    ))
}

unsafe fn ddadd2_vd2_vd2_vd2_avx_sleef(
    x: vdouble2_avx_sleef,
    y: vdouble2_avx_sleef,
) -> vdouble2_avx_sleef {
    let s: vdouble_avx_sleef =
        vadd_vd_vd_vd_avx_sleef(vd2getx_vd_vd2_avx_sleef(x), vd2getx_vd_vd2_avx_sleef(y));
    let v: vdouble_avx_sleef = vsub_vd_vd_vd_avx_sleef(s, vd2getx_vd_vd2_avx_sleef(x));
    let t: vdouble_avx_sleef = vadd_vd_vd_vd_avx_sleef(
        vsub_vd_vd_vd_avx_sleef(vd2getx_vd_vd2_avx_sleef(x), vsub_vd_vd_vd_avx_sleef(s, v)),
        vsub_vd_vd_vd_avx_sleef(vd2getx_vd_vd2_avx_sleef(y), v),
    );
    return vd2setxy_vd2_vd_vd_avx_sleef(
        s,
        vadd_vd_vd_vd_avx_sleef(
            t,
            vadd_vd_vd_vd_avx_sleef(vd2gety_vd_vd2_avx_sleef(x), vd2gety_vd_vd2_avx_sleef(y)),
        ),
    );
}

#[inline]
unsafe fn vlt_vo_vd_vd_avx_sleef(x: vdouble_avx_sleef, y: vdouble_avx_sleef) -> vopmask_avx_sleef {
    vreinterpret_vm_vd_avx_sleef(_mm256_cmp_pd(x, y, _CMP_LT_OQ))
}

#[inline]
unsafe fn vreinterpret_vm_vd_avx_sleef(vd_avx_sleef: vdouble_avx_sleef) -> vmask_avx_sleef {
    _mm256_castpd_si256(vd_avx_sleef)
}

unsafe fn vabs_vd_vd_avx_sleef(d: vdouble_avx_sleef) -> vdouble_avx_sleef {
    _mm256_andnot_pd(_mm256_set1_pd(-0.0), d)
}

unsafe fn vcast_vd_d_avx_sleef(d: f64) -> vdouble_avx_sleef {
    _mm256_set1_pd(d)
}

unsafe fn vrint_vd_vd_avx_sleef(vd_avx_sleef: vdouble_avx_sleef) -> vdouble_avx_sleef {
    _mm256_round_pd(vd_avx_sleef, _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC)
}

unsafe fn vmul_vd_vd_vd_avx_sleef(x: vdouble_avx_sleef, y: vdouble_avx_sleef) -> vdouble_avx_sleef {
    _mm256_mul_pd(x, y)
}

unsafe fn vadd_vi_vi_vi_avx_sleef(x: vint_avx_sleef, y: vint_avx_sleef) -> vint_avx_sleef {
    _mm_add_epi32(x, y)
}

unsafe fn vand_vi_vi_vi_avx_sleef(x: vint_avx_sleef, y: vint_avx_sleef) -> vint_avx_sleef {
    _mm_and_si128(x, y)
}

unsafe fn vmla_vd_vd_vd_vd_avx_sleef(
    x: vdouble_avx_sleef,
    y: vdouble_avx_sleef,
    z: vdouble_avx_sleef,
) -> vdouble_avx_sleef {
    vadd_vd_vd_vd_avx_sleef(vmul_vd_vd_vd_avx_sleef(x, y), z)
}

unsafe fn vadd_vd_vd_vd_avx_sleef(x: vdouble_avx_sleef, y: vdouble_avx_sleef) -> vdouble_avx_sleef {
    _mm256_add_pd(x, y)
}

unsafe fn vrint_vi_vd_avx_sleef(vd_avx_sleef: vdouble_avx_sleef) -> vint_avx_sleef {
    _mm256_cvtpd_epi32(vd_avx_sleef)
}

unsafe fn vtruncate_vd_vd_avx_sleef(vd_avx_sleef: vdouble_avx_sleef) -> vdouble_avx_sleef {
    _mm256_round_pd(vd_avx_sleef, _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC)
}

unsafe fn vmlapn_vd_vd_vd_vd_avx_sleef(
    x: vdouble_avx_sleef,
    y: vdouble_avx_sleef,
    z: vdouble_avx_sleef,
) -> vdouble_avx_sleef {
    return vsub_vd_vd_vd_avx_sleef(vmul_vd_vd_vd_avx_sleef(x, y), z);
}

unsafe fn vsub_vd_vd_vd_avx_sleef(x: vdouble_avx_sleef, y: vdouble_avx_sleef) -> vdouble_avx_sleef {
    _mm256_sub_pd(x, y)
}

unsafe fn ddigeti_vi_ddi_avx_sleef(d: ddi_t_avx_sleef) -> vint_avx_sleef {
    d.i
}

unsafe fn vcast_vi_i_avx_sleef(i: i32) -> vint_avx_sleef {
    _mm_set1_epi32(i)
}

unsafe fn vsel_vi_vo_vi_vi_avx_sleef(
    o: vopmask_avx_sleef,
    x: vint_avx_sleef,
    y: vint_avx_sleef,
) -> vint_avx_sleef {
    return _mm_blendv_epi8(y, x, _mm256_castsi256_si128(o));
}

unsafe fn vcast_vo32_vo64_avx_sleef(o: vopmask_avx_sleef) -> vopmask_avx_sleef {
    return _mm256_castsi128_si256(_mm256_cvtpd_epi32(_mm256_and_pd(
        vreinterpret_vd_vm_avx_sleef(o),
        _mm256_set1_pd(-1.0),
    )));
}

unsafe fn vgt_vo_vd_vd_avx_sleef(x: vdouble_avx_sleef, y: vdouble_avx_sleef) -> vopmask_avx_sleef {
    vreinterpret_vm_vd_avx_sleef(_mm256_cmp_pd(x, y, _CMP_GT_OQ))
}

unsafe fn vreinterpret_vd_vm_avx_sleef(vm: vmask_avx_sleef) -> vdouble_avx_sleef {
    _mm256_castsi256_pd(vm)
}

unsafe fn veq_vo_vi_vi_avx_sleef(x: vint_avx_sleef, y: vint_avx_sleef) -> vopmask_avx_sleef {
    return _mm256_castsi128_si256(_mm_cmpeq_epi32(x, y));
}

unsafe fn vmulsign_vd_vd_vd_avx_sleef(
    x: vdouble_avx_sleef,
    y: vdouble_avx_sleef,
) -> vdouble_avx_sleef {
    vreinterpret_vd_vm_avx_sleef(vxor_vm_vm_vm_avx_sleef(
        vreinterpret_vm_vd_avx_sleef(x),
        vsignbit_vm_vd_avx_sleef(y),
    ))
}

unsafe fn vsignbit_vm_vd_avx_sleef(d: vdouble_avx_sleef) -> vmask_avx_sleef {
    return vand_vm_vm_vm_avx_sleef(
        vreinterpret_vm_vd_avx_sleef(d),
        vreinterpret_vm_vd_avx_sleef(vcast_vd_d_avx_sleef(-0.0)),
    );
}

unsafe fn vand_vm_vm_vm_avx_sleef(x: vmask_avx_sleef, y: vmask_avx_sleef) -> vmask_avx_sleef {
    return vreinterpret_vm_vd_avx_sleef(_mm256_and_pd(
        vreinterpret_vd_vm_avx_sleef(x),
        vreinterpret_vd_vm_avx_sleef(y),
    ));
}
unsafe fn vxor_vm_vm_vm_avx_sleef(x: vmask_avx_sleef, y: vmask_avx_sleef) -> vmask_avx_sleef {
    return vreinterpret_vm_vd_avx_sleef(_mm256_xor_pd(
        vreinterpret_vd_vm_avx_sleef(x),
        vreinterpret_vd_vm_avx_sleef(y),
    ));
}

unsafe fn vor_vm_vo64_vm_avx_sleef(x: vopmask_avx_sleef, y: vmask_avx_sleef) -> vmask_avx_sleef {
    vreinterpret_vm_vd_avx_sleef(_mm256_or_pd(
        vreinterpret_vd_vm_avx_sleef(x),
        vreinterpret_vd_vm_avx_sleef(y),
    ))
}

unsafe fn vand_vm_vo64_vm_avx_sleef(x: vopmask_avx_sleef, y: vmask_avx_sleef) -> vmask_avx_sleef {
    vreinterpret_vm_vd_avx_sleef(_mm256_and_pd(
        vreinterpret_vd_vm_avx_sleef(x),
        vreinterpret_vd_vm_avx_sleef(y),
    ))
}

unsafe fn vandnot_vm_vo64_vm_avx_sleef(
    x: vopmask_avx_sleef,
    y: vmask_avx_sleef,
) -> vmask_avx_sleef {
    vreinterpret_vm_vd_avx_sleef(_mm256_andnot_pd(
        vreinterpret_vd_vm_avx_sleef(x),
        vreinterpret_vd_vm_avx_sleef(y),
    ))
}

unsafe fn vcast_vo64_vo32_avx_sleef(o: vopmask_avx_sleef) -> vopmask_avx_sleef {
    vreinterpret_vm_vd_avx_sleef(_mm256_cmp_pd(
        _mm256_cvtepi32_pd(_mm256_castsi256_si128(o)),
        _mm256_set1_pd(-1.0),
        _CMP_EQ_OQ,
    ))
}

unsafe fn vor_vo_vo_vo_avx_sleef(x: vopmask_avx_sleef, y: vopmask_avx_sleef) -> vopmask_avx_sleef {
    return vreinterpret_vm_vd_avx_sleef(_mm256_or_pd(
        vreinterpret_vd_vm_avx_sleef(x),
        vreinterpret_vd_vm_avx_sleef(y),
    ));
}

unsafe fn visinf_vo_vd_avx_sleef(d: vdouble_avx_sleef) -> vopmask_avx_sleef {
    return vreinterpret_vm_vd_avx_sleef(_mm256_cmp_pd(
        vabs_vd_vd_avx_sleef(d),
        _mm256_set1_pd(f64::INFINITY),
        _CMP_EQ_OQ,
    ));
}

unsafe fn visnan_vo_vd_avx_sleef(d: vdouble_avx_sleef) -> vopmask_avx_sleef {
    return vreinterpret_vm_vd_avx_sleef(_mm256_cmp_pd(d, d, _CMP_NEQ_UQ));
}

unsafe fn visnegzero_vo_vd_avx_sleef(d: vdouble_avx_sleef) -> vopmask_avx_sleef {
    return veq64_vo_vm_vm_avx_sleef(
        vreinterpret_vm_vd_avx_sleef(d),
        vreinterpret_vm_vd_avx_sleef(vcast_vd_d_avx_sleef(-0.0)),
    );
}

unsafe fn veq64_vo_vm_vm_avx_sleef(x: vmask_avx_sleef, y: vmask_avx_sleef) -> vopmask_avx_sleef {
    return vreinterpret_vm_vd_avx_sleef(_mm256_cmp_pd(
        vreinterpret_vd_vm_avx_sleef(vxor_vm_vm_vm_avx_sleef(
            vxor_vm_vm_vm_avx_sleef(x, y),
            vreinterpret_vm_vd_avx_sleef(_mm256_set1_pd(1.0)),
        )),
        _mm256_set1_pd(1.0),
        _CMP_EQ_OQ,
    ));
}

// FIXME: f64::from(1 << 24) might be incorrect
#[inline]
pub unsafe fn Sleef_sind4_u35avx(mut d: __m256d) -> __m256d {
    let mut u = d;
    let mut s = d;
    let r = d;

    let mut ql: __m128i = _mm_setzero_si128();

    if (vtestallones_i_vo64_avx_sleef(vlt_vo_vd_vd_avx_sleef(
        vabs_vd_vd_avx_sleef(d),
        vcast_vd_d_avx_sleef(15.0),
    ))) != 0
    {
        let dql: vdouble_avx_sleef = vrint_vd_vd_avx_sleef(vmul_vd_vd_vd_avx_sleef(
            d,
            vcast_vd_d_avx_sleef(0.318309886183790671537767526745028724),
        ));
        ql = vrint_vi_vd_avx_sleef(dql);
        d = vmla_vd_vd_vd_vd_avx_sleef(dql, vcast_vd_d_avx_sleef(-3.141592653589793116), d);
        d = vmla_vd_vd_vd_vd_avx_sleef(dql, vcast_vd_d_avx_sleef(-1.2246467991473532072e-16), d);
    } else if vtestallones_i_vo64_avx_sleef(vlt_vo_vd_vd_avx_sleef(
        vabs_vd_vd_avx_sleef(d),
        vcast_vd_d_avx_sleef(1e+14),
    )) != 0
    {
        let mut dqh: vdouble_avx_sleef = vtruncate_vd_vd_avx_sleef(vmul_vd_vd_vd_avx_sleef(
            d,
            vcast_vd_d_avx_sleef(0.318309886183790671537767526745028724 / f64::from(1 << 24)),
        ));
        dqh = vmul_vd_vd_vd_avx_sleef(dqh, vcast_vd_d_avx_sleef(f64::from(1 << 24)));
        let dql: vdouble_avx_sleef = vrint_vd_vd_avx_sleef(vmlapn_vd_vd_vd_vd_avx_sleef(
            d,
            vcast_vd_d_avx_sleef(0.318309886183790671537767526745028724),
            dqh,
        ));
        ql = vrint_vi_vd_avx_sleef(dql);

        d = vmla_vd_vd_vd_vd_avx_sleef(dqh, vcast_vd_d_avx_sleef(-3.1415926218032836914), d);
        d = vmla_vd_vd_vd_vd_avx_sleef(dql, vcast_vd_d_avx_sleef(-3.1415926218032836914), d);
        d = vmla_vd_vd_vd_vd_avx_sleef(dqh, vcast_vd_d_avx_sleef(-3.1786509424591713469e-08), d);
        d = vmla_vd_vd_vd_vd_avx_sleef(dql, vcast_vd_d_avx_sleef(-3.1786509424591713469e-08), d);
        d = vmla_vd_vd_vd_vd_avx_sleef(dqh, vcast_vd_d_avx_sleef(-1.2246467864107188502e-16), d);
        d = vmla_vd_vd_vd_vd_avx_sleef(dql, vcast_vd_d_avx_sleef(-1.2246467864107188502e-16), d);
        d = vmla_vd_vd_vd_vd_avx_sleef(
            vadd_vd_vd_vd_avx_sleef(dqh, dql),
            vcast_vd_d_avx_sleef(-1.2736634327021899816e-24),
            d,
        );
    } else {
        let mut ddi_avx_sleef: ddi_t_avx_sleef = rempi_avx_sleef(d);
        ql = vand_vi_vi_vi_avx_sleef(
            ddigeti_vi_ddi_avx_sleef(ddi_avx_sleef),
            vcast_vi_i_avx_sleef(3),
        );
        ql = vadd_vi_vi_vi_avx_sleef(
            vadd_vi_vi_vi_avx_sleef(ql, ql),
            vsel_vi_vo_vi_vi_avx_sleef(
                vcast_vo32_vo64_avx_sleef(vgt_vo_vd_vd_avx_sleef(
                    vd2getx_vd_vd2_avx_sleef(ddigetdd_vd2_ddi_avx_sleef(ddi_avx_sleef)),
                    vcast_vd_d_avx_sleef(0.0),
                )),
                vcast_vi_i_avx_sleef(2),
                vcast_vi_i_avx_sleef(1),
            ),
        );
        ql = vsra_vi_vi_i_avx_sleef!(ql, 2);
        let o: vopmask_avx_sleef = veq_vo_vi_vi_avx_sleef(
            vand_vi_vi_vi_avx_sleef(
                ddigeti_vi_ddi_avx_sleef(ddi_avx_sleef),
                vcast_vi_i_avx_sleef(1),
            ),
            vcast_vi_i_avx_sleef(1),
        );
        let mut x: vdouble2_avx_sleef = vcast_vd2_vd_vd_avx_sleef(
            vmulsign_vd_vd_vd_avx_sleef(
                vcast_vd_d_avx_sleef(-3.141592653589793116 * 0.5),
                vd2getx_vd_vd2_avx_sleef(ddigetdd_vd2_ddi_avx_sleef(ddi_avx_sleef)),
            ),
            vmulsign_vd_vd_vd_avx_sleef(
                vcast_vd_d_avx_sleef(-1.2246467991473532072e-16 * 0.5),
                vd2getx_vd_vd2_avx_sleef(ddigetdd_vd2_ddi_avx_sleef(ddi_avx_sleef)),
            ),
        );
        x = ddadd2_vd2_vd2_vd2_avx_sleef(ddigetdd_vd2_ddi_avx_sleef(ddi_avx_sleef), x);
        ddi_avx_sleef = ddisetdd_ddi_ddi_vd2_avx_sleef(
            ddi_avx_sleef,
            vsel_vd2_vo_vd2_vd2_avx_sleef(
                vcast_vo64_vo32_avx_sleef(o),
                x,
                ddigetdd_vd2_ddi_avx_sleef(ddi_avx_sleef),
            ),
        );
        d = vadd_vd_vd_vd_avx_sleef(
            vd2getx_vd_vd2_avx_sleef(ddigetdd_vd2_ddi_avx_sleef(ddi_avx_sleef)),
            vd2gety_vd_vd2_avx_sleef(ddigetdd_vd2_ddi_avx_sleef(ddi_avx_sleef)),
        );
        d = vreinterpret_vd_vm_avx_sleef(vor_vm_vo64_vm_avx_sleef(
            vor_vo_vo_vo_avx_sleef(visinf_vo_vd_avx_sleef(r), visnan_vo_vd_avx_sleef(r)),
            vreinterpret_vm_vd_avx_sleef(d),
        ));
    }

    s = vmul_vd_vd_vd_avx_sleef(d, d);

    d = vreinterpret_vd_vm_avx_sleef(vxor_vm_vm_vm_avx_sleef(
        vand_vm_vo64_vm_avx_sleef(
            vcast_vo64_vo32_avx_sleef(veq_vo_vi_vi_avx_sleef(
                vand_vi_vi_vi_avx_sleef(ql, vcast_vi_i_avx_sleef(1)),
                vcast_vi_i_avx_sleef(1),
            )),
            vreinterpret_vm_vd_avx_sleef(vcast_vd_d_avx_sleef(-0.0)),
        ),
        vreinterpret_vm_vd_avx_sleef(d),
    ));

    let s2: vdouble_avx_sleef = vmul_vd_vd_vd_avx_sleef(s, s);
    let s4: vdouble_avx_sleef = vmul_vd_vd_vd_avx_sleef(s2, s2);

    u = vmla_vd_vd_vd_vd_avx_sleef(
        s4,
        vmla_vd_vd_vd_vd_avx_sleef(
            s2,
            vmla_vd_vd_vd_vd_avx_sleef(
                s,
                vcast_vd_d_avx_sleef(-7.97255955009037868891952e-18),
                vcast_vd_d_avx_sleef(2.81009972710863200091251e-15),
            ),
            vmla_vd_vd_vd_vd_avx_sleef(
                s,
                vcast_vd_d_avx_sleef(-7.64712219118158833288484e-13),
                vcast_vd_d_avx_sleef(1.60590430605664501629054e-10),
            ),
        ),
        vmla_vd_vd_vd_vd_avx_sleef(
            s2,
            vmla_vd_vd_vd_vd_avx_sleef(
                s,
                vcast_vd_d_avx_sleef(-2.50521083763502045810755e-08),
                vcast_vd_d_avx_sleef(2.75573192239198747630416e-06),
            ),
            vmla_vd_vd_vd_vd_avx_sleef(
                s,
                vcast_vd_d_avx_sleef(-0.000198412698412696162806809),
                vcast_vd_d_avx_sleef(0.00833333333333332974823815),
            ),
        ),
    );

    u = vmla_vd_vd_vd_vd_avx_sleef(u, s, vcast_vd_d_avx_sleef(-0.166666666666666657414808));

    u = vadd_vd_vd_vd_avx_sleef(vmul_vd_vd_vd_avx_sleef(s, vmul_vd_vd_vd_avx_sleef(u, d)), d);

    u = vsel_vd_vo_vd_vd_avx_sleef(visnegzero_vo_vd_avx_sleef(r), r, u);

    u
}

// FIXME:
// - table indexing might not be correct
unsafe fn rempi_avx_sleef(mut a: vdouble_avx_sleef) -> ddi_t_avx_sleef {
    let mut ex: vint_avx_sleef = vilogb2k_vi_vd_avx_sleef(a);

    ex = vsub_vi_vi_vi_avx_sleef(ex, vcast_vi_i_avx_sleef(55));
    let mut q: vint_avx_sleef = vand_vi_vo_vi_avx_sleef(
        vgt_vo_vi_vi_avx_sleef(ex, vcast_vi_i_avx_sleef(700 - 55)),
        vcast_vi_i_avx_sleef(-64),
    );
    a = vldexp3_vd_vd_vi_avx_sleef(a, q);
    ex = vandnot_vi_vi_vi_avx_sleef(vsra_vi_vi_i_avx_sleef!(ex, 31), ex);
    ex = vsll_vi_vi_i_avx_sleef!(ex, 2);
    let mut x = ddmul_vd2_vd_vd_avx_sleef(a, vgather_vd_p_vi_avx_sleef(Sleef_rempitabdp, ex));
    let mut di: di_t_avx_sleef = rempisub_avx_sleef(vd2getx_vd_vd2_avx_sleef(x));
    q = di.i; // digeti_vi_di_avx_sleef(di);
    x.x = di.d; // x = vd2setx_vd2_vd2_vd_avx_sleef(x, di.d);
    x = ddnormalize_vd2_vd2_avx_sleef(x);
    let mut y = ddmul_vd2_vd_vd_avx_sleef(a, vgather_vd_p_vi_avx_sleef(&Sleef_rempitabdp[1..], ex));
    x = ddadd2_vd2_vd2_vd2_avx_sleef(x, y);
    di = rempisub_avx_sleef(vd2getx_vd_vd2_avx_sleef(x));
    q = vadd_vi_vi_vi_avx_sleef(q, di.i);
    x.x = di.d; //x = vd2setx_vd2_vd2_vd_avx_sleef(x, di.d);
    x = ddnormalize_vd2_vd2_avx_sleef(x);
    y = vcast_vd2_vd_vd_avx_sleef(
        vgather_vd_p_vi_avx_sleef(&Sleef_rempitabdp[2..], ex),
        vgather_vd_p_vi_avx_sleef(&Sleef_rempitabdp[3..], ex),
    );
    y = ddmul_vd2_vd2_vd_avx_sleef(y, a);
    x = ddadd2_vd2_vd2_vd2_avx_sleef(x, y);
    x = ddnormalize_vd2_vd2_avx_sleef(x);
    x = ddmul_vd2_vd2_vd2_avx_sleef(
        x,
        vcast_vd2_d_d_avx_sleef(3.141592653589793116 * 2.0, 1.2246467991473532072e-16 * 2.0),
    );
    let o: vopmask_avx_sleef =
        vlt_vo_vd_vd_avx_sleef(vabs_vd_vd_avx_sleef(a), vcast_vd_d_avx_sleef(0.7));
    x.x = vsel_vd_vo_vd_vd_avx_sleef(o, a, vd2getx_vd_vd2_avx_sleef(x));
    x.y = vreinterpret_vd_vm_avx_sleef(vandnot_vm_vo64_vm_avx_sleef(
        o,
        vreinterpret_vm_vd_avx_sleef(vd2gety_vd_vd2_avx_sleef(x)),
    ));

    ddi_t_avx_sleef {
        // ddisetddi_ddi_vd2_vi_avx_sleef(x, q)
        dd_avx_sleef: x,
        i: q,
    }
}

unsafe fn vilogb2k_vi_vd_avx_sleef(d: vdouble_avx_sleef) -> vint_avx_sleef {
    let mut q = vcastu_vi_vm_avx_sleef(vreinterpret_vm_vd_avx_sleef(d));
    q = vsrl_vi_vi_i_avx_sleef!(q, 20);
    q = vand_vi_vi_vi_avx_sleef(q, vcast_vi_i_avx_sleef(0x7ff));
    q = vsub_vi_vi_vi_avx_sleef(q, vcast_vi_i_avx_sleef(0x3ff));
    return q;
}

unsafe fn vcastu_vi_vm_avx_sleef(vi: vmask_avx_sleef) -> vint_avx_sleef {
    return _mm_or_si128(
        _mm_and_si128(
            _mm_shuffle_epi32(_mm256_castsi256_si128(vi), 0x0d),
            _mm_set_epi32(0, 0, -1, -1),
        ),
        _mm_and_si128(
            _mm_shuffle_epi32(_mm256_extractf128_si256(vi, 1), 0xd0),
            _mm_set_epi32(-1, -1, 0, 0),
        ),
    );
}

unsafe fn vsub_vi_vi_vi_avx_sleef(x: vint_avx_sleef, y: vint_avx_sleef) -> vint_avx_sleef {
    return _mm_sub_epi32(x, y);
}

unsafe fn vand_vi_vo_vi_avx_sleef(m: vopmask_avx_sleef, y: vint_avx_sleef) -> vint_avx_sleef {
    return _mm_and_si128(_mm256_castsi256_si128(m), y);
}

unsafe fn vgt_vo_vi_vi_avx_sleef(x: vint_avx_sleef, y: vint_avx_sleef) -> vopmask_avx_sleef {
    return _mm256_castsi128_si256(_mm_cmpgt_epi32(x, y));
}

unsafe fn vldexp3_vd_vd_vi_avx_sleef(d: vdouble_avx_sleef, q: vint_avx_sleef) -> vdouble_avx_sleef {
    return vreinterpret_vd_vm_avx_sleef(vadd64_vm_vm_vm_avx_sleef(
        vreinterpret_vm_vd_avx_sleef(d),
        vcastu_vm_vi_avx_sleef(vsll_vi_vi_i_avx_sleef!(q, 20)),
    ));
}

unsafe fn vcastu_vm_vi_avx_sleef(vi: vint_avx_sleef) -> vmask_avx_sleef {
    let m = _mm256_castsi128_si256(_mm_and_si128(
        _mm_shuffle_epi32(vi, 0x40),
        _mm_set_epi32(-1, 0, -1, 0),
    ));
    return _mm256_insertf128_si256(
        m,
        _mm_and_si128(_mm_shuffle_epi32(vi, 0xc8), _mm_set_epi32(-1, 0, -1, 0)),
        1,
    );
}

unsafe fn vadd64_vm_vm_vm_avx_sleef(x: vmask_avx_sleef, y: vmask_avx_sleef) -> vmask_avx_sleef {
    let ix = vcast_vi2_vm_avx_sleef(x);
    let iy = vcast_vi2_vm_avx_sleef(y);

    let iz = vint2_avx_sleef {
        x: _mm_add_epi64(ix.x, iy.x),
        y: _mm_add_epi64(ix.y, iy.y),
    };

    vcast_vm_vi2_avx_sleef(iz)
}

unsafe fn vcast_vi2_vm_avx_sleef(vm: vmask_avx_sleef) -> vint2_avx_sleef {
    vint2_avx_sleef {
        x: _mm256_castsi256_si128(vm),
        y: _mm256_extractf128_si256(vm, 1),
    }
}

unsafe fn vcast_vm_vi2_avx_sleef(vi: vint2_avx_sleef) -> vmask_avx_sleef {
    let mut m: vmask_avx_sleef = _mm256_castsi128_si256(vi.x);
    m = _mm256_insertf128_si256(m, vi.y, 1);
    return m;
}

unsafe fn vandnot_vi_vi_vi_avx_sleef(x: vint_avx_sleef, y: vint_avx_sleef) -> vint_avx_sleef {
    return _mm_andnot_si128(x, y);
}

unsafe fn ddmul_vd2_vd_vd_avx_sleef(
    x: vdouble_avx_sleef,
    y: vdouble_avx_sleef,
) -> vdouble2_avx_sleef {
    let xh = vupper_vd_vd_avx_sleef(x);
    let xl = vsub_vd_vd_vd_avx_sleef(x, xh);
    let yh = vupper_vd_vd_avx_sleef(y);
    let yl = vsub_vd_vd_vd_avx_sleef(y, yh);

    let s = vmul_vd_vd_vd_avx_sleef(x, y);
    return vd2setxy_vd2_vd_vd_avx_sleef(
        s,
        vadd_vd_5vd_avx_sleef(
            vmul_vd_vd_vd_avx_sleef(xh, yh),
            vneg_vd_vd_avx_sleef(s),
            vmul_vd_vd_vd_avx_sleef(xl, yh),
            vmul_vd_vd_vd_avx_sleef(xh, yl),
            vmul_vd_vd_vd_avx_sleef(xl, yl),
        ),
    );
}

unsafe fn ddmul_vd2_vd2_vd2_avx_sleef(
    x: vdouble2_avx_sleef,
    y: vdouble2_avx_sleef,
) -> vdouble2_avx_sleef {
    let xh = vupper_vd_vd_avx_sleef(vd2getx_vd_vd2_avx_sleef(x));
    let xl = vsub_vd_vd_vd_avx_sleef(vd2getx_vd_vd2_avx_sleef(x), xh);
    let yh = vupper_vd_vd_avx_sleef(vd2getx_vd_vd2_avx_sleef(y));
    let yl = vsub_vd_vd_vd_avx_sleef(vd2getx_vd_vd2_avx_sleef(y), yh);

    let s = vmul_vd_vd_vd_avx_sleef(vd2getx_vd_vd2_avx_sleef(x), vd2getx_vd_vd2_avx_sleef(y));
    return vd2setxy_vd2_vd_vd_avx_sleef(
        s,
        vadd_vd_7vd_avx_sleef(
            vmul_vd_vd_vd_avx_sleef(xh, yh),
            vneg_vd_vd_avx_sleef(s),
            vmul_vd_vd_vd_avx_sleef(xl, yh),
            vmul_vd_vd_vd_avx_sleef(xh, yl),
            vmul_vd_vd_vd_avx_sleef(xl, yl),
            vmul_vd_vd_vd_avx_sleef(vd2getx_vd_vd2_avx_sleef(x), vd2gety_vd_vd2_avx_sleef(y)),
            vmul_vd_vd_vd_avx_sleef(vd2gety_vd_vd2_avx_sleef(x), vd2getx_vd_vd2_avx_sleef(y)),
        ),
    );
}

// FIXME: overflowing literals correct?
#[allow(overflowing_literals)]
unsafe fn vupper_vd_vd_avx_sleef(d: vdouble_avx_sleef) -> vdouble_avx_sleef {
    return vreinterpret_vd_vm_avx_sleef(vand_vm_vm_vm_avx_sleef(
        vreinterpret_vm_vd_avx_sleef(d),
        vcast_vm_i_i_avx_sleef(0xffffffff, 0xf8000000),
    ));
}

unsafe fn vcast_vm_i_i_avx_sleef(i0: i32, i1: i32) -> vmask_avx_sleef {
    return _mm256_set_epi32(i0, i1, i0, i1, i0, i1, i0, i1);
}

unsafe fn vadd_vd_3vd_avx_sleef(
    v0: vdouble_avx_sleef,
    v1: vdouble_avx_sleef,
    v2: vdouble_avx_sleef,
) -> vdouble_avx_sleef {
    return vadd_vd_vd_vd_avx_sleef(vadd_vd_vd_vd_avx_sleef(v0, v1), v2);
}

unsafe fn vadd_vd_4vd_avx_sleef(
    v0: vdouble_avx_sleef,
    v1: vdouble_avx_sleef,
    v2: vdouble_avx_sleef,
    v3: vdouble_avx_sleef,
) -> vdouble_avx_sleef {
    return vadd_vd_3vd_avx_sleef(vadd_vd_vd_vd_avx_sleef(v0, v1), v2, v3);
}

unsafe fn vadd_vd_5vd_avx_sleef(
    v0: vdouble_avx_sleef,
    v1: vdouble_avx_sleef,
    v2: vdouble_avx_sleef,
    v3: vdouble_avx_sleef,
    v4: vdouble_avx_sleef,
) -> vdouble_avx_sleef {
    return vadd_vd_4vd_avx_sleef(vadd_vd_vd_vd_avx_sleef(v0, v1), v2, v3, v4);
}

unsafe fn vadd_vd_6vd_avx_sleef(
    v0: vdouble_avx_sleef,
    v1: vdouble_avx_sleef,
    v2: vdouble_avx_sleef,
    v3: vdouble_avx_sleef,
    v4: vdouble_avx_sleef,
    v5: vdouble_avx_sleef,
) -> vdouble_avx_sleef {
    return vadd_vd_5vd_avx_sleef(vadd_vd_vd_vd_avx_sleef(v0, v1), v2, v3, v4, v5);
}

unsafe fn vadd_vd_7vd_avx_sleef(
    v0: vdouble_avx_sleef,
    v1: vdouble_avx_sleef,
    v2: vdouble_avx_sleef,
    v3: vdouble_avx_sleef,
    v4: vdouble_avx_sleef,
    v5: vdouble_avx_sleef,
    v6: vdouble_avx_sleef,
) -> vdouble_avx_sleef {
    return vadd_vd_6vd_avx_sleef(vadd_vd_vd_vd_avx_sleef(v0, v1), v2, v3, v4, v5, v6);
}

unsafe fn vneg_vd_vd_avx_sleef(d: vdouble_avx_sleef) -> vdouble_avx_sleef {
    return _mm256_xor_pd(_mm256_set1_pd(-0.0), d);
}

// FIXME: might not be correct
unsafe fn vgather_vd_p_vi_avx_sleef(ptr: &[f64], vi: vint_avx_sleef) -> vdouble_avx_sleef {
    let mut a = [0; 1 << 2];
    vstoreu_v_p_vi_avx_sleef(a.as_mut_ptr(), vi);
    return _mm256_set_pd(
        ptr[a[3] as usize],
        ptr[a[2] as usize],
        ptr[a[1] as usize],
        ptr[a[0] as usize],
    );
}

unsafe fn vstoreu_v_p_vi_avx_sleef(p: *mut i32, v: vint_avx_sleef) {
    _mm_storeu_si128(p as *mut __m128i, v);
}

unsafe fn rempisub_avx_sleef(x: vdouble_avx_sleef) -> di_t_avx_sleef {
    let y = vrint_vd_vd_avx_sleef(vmul_vd_vd_vd_avx_sleef(x, vcast_vd_d_avx_sleef(4.0)));
    let vi = vtruncate_vi_vd_avx_sleef(vsub_vd_vd_vd_avx_sleef(
        y,
        vmul_vd_vd_vd_avx_sleef(vrint_vd_vd_avx_sleef(x), vcast_vd_d_avx_sleef(4.0)),
    ));
    return disetdi_di_vd_vi_avx_sleef(
        vsub_vd_vd_vd_avx_sleef(x, vmul_vd_vd_vd_avx_sleef(y, vcast_vd_d_avx_sleef(0.25))),
        vi,
    );
}

unsafe fn vtruncate_vi_vd_avx_sleef(vd_avx_sleef: vdouble_avx_sleef) -> vint_avx_sleef {
    return _mm256_cvttpd_epi32(vd_avx_sleef);
}

unsafe fn ddnormalize_vd2_vd2_avx_sleef(t: vdouble2_avx_sleef) -> vdouble2_avx_sleef {
    let s = vadd_vd_vd_vd_avx_sleef(vd2getx_vd_vd2_avx_sleef(t), vd2gety_vd_vd2_avx_sleef(t));
    return vd2setxy_vd2_vd_vd_avx_sleef(
        s,
        vadd_vd_vd_vd_avx_sleef(
            vsub_vd_vd_vd_avx_sleef(vd2getx_vd_vd2_avx_sleef(t), s),
            vd2gety_vd_vd2_avx_sleef(t),
        ),
    );
}

unsafe fn ddmul_vd2_vd2_vd_avx_sleef(
    x: vdouble2_avx_sleef,
    y: vdouble_avx_sleef,
) -> vdouble2_avx_sleef {
    let xh = vupper_vd_vd_avx_sleef(vd2getx_vd_vd2_avx_sleef(x));
    let xl = vsub_vd_vd_vd_avx_sleef(vd2getx_vd_vd2_avx_sleef(x), xh);
    let yh = vupper_vd_vd_avx_sleef(y);
    let yl = vsub_vd_vd_vd_avx_sleef(y, yh);

    let s = vmul_vd_vd_vd_avx_sleef(vd2getx_vd_vd2_avx_sleef(x), y);
    return vd2setxy_vd2_vd_vd_avx_sleef(
        s,
        vadd_vd_6vd_avx_sleef(
            vmul_vd_vd_vd_avx_sleef(xh, yh),
            vneg_vd_vd_avx_sleef(s),
            vmul_vd_vd_vd_avx_sleef(xl, yh),
            vmul_vd_vd_vd_avx_sleef(xh, yl),
            vmul_vd_vd_vd_avx_sleef(xl, yl),
            vmul_vd_vd_vd_avx_sleef(vd2gety_vd_vd2_avx_sleef(x), y),
        ),
    );
}

unsafe fn vcast_vd2_d_d_avx_sleef(h: f64, l: f64) -> vdouble2_avx_sleef {
    return vd2setxy_vd2_vd_vd_avx_sleef(vcast_vd_d_avx_sleef(h), vcast_vd_d_avx_sleef(l));
}
