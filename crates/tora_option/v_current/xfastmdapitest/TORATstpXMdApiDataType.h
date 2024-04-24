/////////////////////////////////////////////////////////////////////////
///@company 上海泰琰信息科技有限公司
///@file TORATstpXMdApiDataType.h
///@brief 定义了客户端接口使用的业务数据类型
///@history 
/////////////////////////////////////////////////////////////////////////
#ifndef TORATSTPXMDAPIDATATYPE_H__
#define TORATSTPXMDAPIDATATYPE_H__


#include <limits.h>
#include <float.h>

namespace TORALEV1API
{
	///定义空值
	const int INT_NULL_VAL = INT_MAX;
	const double FLOAT_NULL_VAL = DBL_MAX;
	const char CHAR_NULL_VAL = 0;
	const short WORD_NULL_VAL = SHRT_MAX;
#ifdef WINDOWS
	const __int64 LONG_NULL_VAL = _I64_MAX;
#else
	const long long LONG_NULL_VAL = LLONG_MAX;
#endif
	
	///整型数据置空和判空
	inline void set_null(int &v)
	{
		v = INT_NULL_VAL;
	}
	inline bool is_null(const int &v)
	{
		return v == INT_NULL_VAL;
	}
	
	///浮点型数据置空和判空
	inline void set_null(double &v)
	{
		v = FLOAT_NULL_VAL;
	}
	inline bool is_null(const double &v)
	{
		return v == FLOAT_NULL_VAL;
	}
	
	///字符型数据置空和判空
	inline void set_null(char &v)
	{
		v = CHAR_NULL_VAL;
	}
	inline bool is_null(const char &v)
	{
		return v == CHAR_NULL_VAL;
	}
	
	///短整型数据置空和判空
	inline void set_null(short &v)
	{
		v = WORD_NULL_VAL;
	}
	inline bool is_null(const short &v)
	{
		return v == WORD_NULL_VAL;
	}
	
	///长整型数据置空和判空
	#ifdef WINDOWS
	inline void set_null(__int64 &v)
	#else
	inline void set_null(long long &v)
	#endif
	{
		v = LONG_NULL_VAL;
	}
	#ifdef WINDOWS
	inline bool is_null(const __int64 &v)
	#else
	inline bool is_null(const long long &v)
	#endif
	{
		return v == LONG_NULL_VAL;
	}
	
	///字符串型数据置空和判空
	inline void set_null(char *v)
	{
		v[0] = '\0';
	}
	inline bool is_null(const char *v)
	{
		const char *p=v;
		while (*p)
		{
			if (*p!=' ')
			{
				return false;
			}
			p++;
		}
		return true;
	}
	
	enum TORA_TE_RESUME_TYPE
	{
		TORA_TERT_RESTART = 0,
		TORA_TERT_RESUME,
		TORA_TERT_QUICK
	};
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpLogInAccountType是一个登录账户类型
	/////////////////////////////////////////////////////////////////////////
	typedef char TTORATstpLogInAccountType[21];
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpLogInAccountTypeType是一个登录账户类型类型
	/////////////////////////////////////////////////////////////////////////
	///用户代码
	const char TORA_TSTP_LACT_UserID = '0';
	///资金账号
	const char TORA_TSTP_LACT_AccountID = '1';
	///上海A股
	const char TORA_TSTP_LACT_SHAStock = '2';
	///深圳A股
	const char TORA_TSTP_LACT_SZAStock = '3';
	///上海B股
	const char TORA_TSTP_LACT_SHBStock = '4';
	///深圳B股
	const char TORA_TSTP_LACT_SZBStock = '5';
	///三板A
	const char TORA_TSTP_LACT_ThreeNewBoardA = '6';
	///三板B
	const char TORA_TSTP_LACT_ThreeNewBoardB = '7';
	///港股
	const char TORA_TSTP_LACT_HKStock = '8';
	///统一用户代码
	const char TORA_TSTP_LACT_UnifiedUserID = '9';
	typedef char TTORATstpLogInAccountTypeType;
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpDepartmentIDType是一个部门代码类型
	/////////////////////////////////////////////////////////////////////////
	typedef char TTORATstpDepartmentIDType[11];
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpAuthModeType是一个认证方式类型
	/////////////////////////////////////////////////////////////////////////
	///密码
	const char TORA_TSTP_AM_Password = '0';
	///指纹
	const char TORA_TSTP_AM_FingerPrint = '1';
	///钥匙串
	const char TORA_TSTP_AM_CertInfo = '2';
	typedef char TTORATstpAuthModeType;
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpPasswordType是一个密码类型
	/////////////////////////////////////////////////////////////////////////
	typedef char TTORATstpPasswordType[41];
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpUserProductInfoType是一个用户端产品信息类型
	/////////////////////////////////////////////////////////////////////////
	typedef char TTORATstpUserProductInfoType[11];
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpInterfaceProductInfoType是一个接口端产品信息类型
	/////////////////////////////////////////////////////////////////////////
	typedef char TTORATstpInterfaceProductInfoType[33];
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpTerminalInfoType是一个终端信息类型
	/////////////////////////////////////////////////////////////////////////
	typedef char TTORATstpTerminalInfoType[256];
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpIPAddressType是一个IP地址类型
	/////////////////////////////////////////////////////////////////////////
	typedef char TTORATstpIPAddressType[16];
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpMacAddressType是一个Mac地址类型
	/////////////////////////////////////////////////////////////////////////
	typedef char TTORATstpMacAddressType[21];
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpLangType是一个语言类型
	/////////////////////////////////////////////////////////////////////////
	///简体中文
	const char TORA_TSTP_LGT_ZHCN = '0';
	///中文香港
	const char TORA_TSTP_LGT_ZHHK = '1';
	///英文美国
	const char TORA_TSTP_LGT_ENUS = '2';
	typedef char TTORATstpLangType;
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpDeviceIDType是一个设备标识类型
	/////////////////////////////////////////////////////////////////////////
	typedef char TTORATstpDeviceIDType[129];
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpCertSerialType是一个认证序列类型
	/////////////////////////////////////////////////////////////////////////
	typedef char TTORATstpCertSerialType[129];
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpDeviceTypeType是一个设备类别类型
	/////////////////////////////////////////////////////////////////////////
	///PC端
	const char TORA_TSTP_DT_PC = '0';
	///移动端
	const char TORA_TSTP_DT_Mobile = '1';
	typedef char TTORATstpDeviceTypeType;
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpFrontIDType是一个前置编号类型
	/////////////////////////////////////////////////////////////////////////
	typedef int TTORATstpFrontIDType;
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpSessionIDType是一个会话编号类型
	/////////////////////////////////////////////////////////////////////////
	typedef int TTORATstpSessionIDType;
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpOrderRefType是一个报单引用类型
	/////////////////////////////////////////////////////////////////////////
	typedef int TTORATstpOrderRefType;
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpVolumeType是一个数量类型
	/////////////////////////////////////////////////////////////////////////
	typedef int TTORATstpVolumeType;
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpTimeType是一个时间类型
	/////////////////////////////////////////////////////////////////////////
	typedef char TTORATstpTimeType[9];
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpSystemNameType是一个系统名称类型
	/////////////////////////////////////////////////////////////////////////
	typedef char TTORATstpSystemNameType[41];
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpDateType是一个日期类型
	/////////////////////////////////////////////////////////////////////////
	typedef char TTORATstpDateType[9];
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpUserIDType是一个交易用户代码类型
	/////////////////////////////////////////////////////////////////////////
	typedef char TTORATstpUserIDType[16];
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpUserNameType是一个用户名称类型
	/////////////////////////////////////////////////////////////////////////
	typedef char TTORATstpUserNameType[81];
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpUserTypeType是一个用户类型类型
	/////////////////////////////////////////////////////////////////////////
	///经纪公司用户
	const char TORA_TSTP_UTYPE_BrokerUser = '0';
	///超级用户
	const char TORA_TSTP_UTYPE_SuperUser = '1';
	///投资者用户
	const char TORA_TSTP_UTYPE_Investor = '2';
	typedef char TTORATstpUserTypeType;
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpCommFluxType是一个通讯流量类型
	/////////////////////////////////////////////////////////////////////////
	typedef int TTORATstpCommFluxType;
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpBoolType是一个布尔型类型
	/////////////////////////////////////////////////////////////////////////
	typedef int TTORATstpBoolType;
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpErrorIDType是一个错误代码类型
	/////////////////////////////////////////////////////////////////////////
	typedef int TTORATstpErrorIDType;
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpErrorMsgType是一个错误信息类型
	/////////////////////////////////////////////////////////////////////////
	typedef char TTORATstpErrorMsgType[81];
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpExchangeIDType是一个交易所代码类型
	/////////////////////////////////////////////////////////////////////////
	///上海交易所
	const char TORA_TSTP_EXD_SSE = '1';
	///深圳交易所
	const char TORA_TSTP_EXD_SZSE = '2';
	///香港交易所
	const char TORA_TSTP_EXD_HK = '3';
	///北京证券交易所
	const char TORA_TSTP_EXD_BSE = '4';
	typedef char TTORATstpExchangeIDType;
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpSecurityIDType是一个证券代码类型
	/////////////////////////////////////////////////////////////////////////
	typedef char TTORATstpSecurityIDType[31];
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpMarketIDType是一个市场代码类型
	/////////////////////////////////////////////////////////////////////////
	///上海A股
	const char TORA_TSTP_MKD_SHA = '1';
	///深圳A股
	const char TORA_TSTP_MKD_SZA = '2';
	///上海B股
	const char TORA_TSTP_MKD_SHB = '3';
	///深圳B股
	const char TORA_TSTP_MKD_SZB = '4';
	///深圳三版A股
	const char TORA_TSTP_MKD_SZThreeA = '5';
	///深圳三版B股
	const char TORA_TSTP_MKD_SZThreeB = '6';
	///境外市场
	const char TORA_TSTP_MKD_Foreign = '7';
	///深圳港股通市场
	const char TORA_TSTP_MKD_SZHK = '8';
	///上海港股通市场
	const char TORA_TSTP_MKD_SHHK = '9';
	///北京主板
	const char TORA_TSTP_MKD_BJMain = 'a';
	typedef char TTORATstpMarketIDType;
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpSecurityNameType是一个证券名称类型
	/////////////////////////////////////////////////////////////////////////
	typedef char TTORATstpSecurityNameType[81];
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpPriceType是一个价格类型
	/////////////////////////////////////////////////////////////////////////
	typedef double TTORATstpPriceType;
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpLongVolumeType是一个LongVolume类型
	/////////////////////////////////////////////////////////////////////////
	#ifdef WINDOWS
	typedef __int64 TTORATstpLongVolumeType;
	#else
	typedef long long int TTORATstpLongVolumeType;
	#endif
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpMoneyType是一个资金类型
	/////////////////////////////////////////////////////////////////////////
	typedef double TTORATstpMoneyType;
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpRatioType是一个比率类型
	/////////////////////////////////////////////////////////////////////////
	typedef double TTORATstpRatioType;
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpLargeVolumeType是一个大额数量类型
	/////////////////////////////////////////////////////////////////////////
	typedef double TTORATstpLargeVolumeType;
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpMillisecType是一个时间（毫秒）类型
	/////////////////////////////////////////////////////////////////////////
	typedef int TTORATstpMillisecType;
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpMDSecurityStatType是一个行情证券状态类型
	/////////////////////////////////////////////////////////////////////////
	///开盘前
	const char TORA_TSTP_MSST_PreOpen = '0';
	///集合竞价
	const char TORA_TSTP_MSST_CallAuction = '1';
	///连续交易
	const char TORA_TSTP_MSST_Continous = '2';
	///休市
	const char TORA_TSTP_MSST_Pause = '3';
	///停牌
	const char TORA_TSTP_MSST_Suspend = '4';
	///长期停牌
	const char TORA_TSTP_MSST_LongSuspend = '5';
	///波动性中断
	const char TORA_TSTP_MSST_UndulationInt = '6';
	///熔断可恢复
	const char TORA_TSTP_MSST_CircuitBreak = '7';
	///熔断不可恢复
	const char TORA_TSTP_MSST_CircuitBreakU = '8';
	///闭市
	const char TORA_TSTP_MSST_Close = '9';
	///其它
	const char TORA_TSTP_MSST_Other = 'a';
	///收盘集合竞价
	const char TORA_TSTP_MSST_CloseCallAuction = 'b';
	///集中撮合(盘后定价)
	const char TORA_TSTP_MSST_CallMatch = 'c';
	///连续交易(盘后定价)
	const char TORA_TSTP_MSST_PostContinous = 'd';
	///闭市(盘后定价)
	const char TORA_TSTP_MSST_PostClose = 'e';
	///开盘前(盘后定价)
	const char TORA_TSTP_MSST_PrePostOpen = 'f';
	typedef char TTORATstpMDSecurityStatType;
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpHWLevelType是一个警示级别类型
	/////////////////////////////////////////////////////////////////////////
	typedef int TTORATstpHWLevelType;
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpEndFlagType是一个结束标志类型
	/////////////////////////////////////////////////////////////////////////
	///待续
	const char TORA_TSTP_EF_ToBeContinued = '1';
	///批结束
	const char TORA_TSTP_EF_BatchEnd = '2';
	///全部完成
	const char TORA_TSTP_EF_Completed = '3';
	///无数据
	const char TORA_TSTP_EF_NOP = '4';
	typedef char TTORATstpEndFlagType;
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpMarketStatusType是一个市场状态类型
	/////////////////////////////////////////////////////////////////////////
	///未知
	const char TORA_TSTP_MST_UnKnown = '#';
	///开盘前
	const char TORA_TSTP_MST_BeforeTrading = '0';
	///连续交易
	const char TORA_TSTP_MST_Continous = '1';
	///收盘
	const char TORA_TSTP_MST_Closed = '2';
	///开盘集合竞价
	const char TORA_TSTP_MST_OpenCallAuction = '3';
	///(港股通)未开市
	const char TORA_TSTP_MST_SZSEHKUnopened = 'a';
	///(港股通)开盘集合竞价输入买卖盘
	const char TORA_TSTP_MST_SZSEHKOpenCallAuctionInput = 'b';
	///(港股通)开盘集合竞价对盘前
	const char TORA_TSTP_MST_SZSEHKOpenCallAuctionBeforeMatch = 'c';
	///(港股通)开盘集合竞价对盘
	const char TORA_TSTP_MST_SZSEHKOpenCallAuctionMatch = 'd';
	///(港股通)暂停
	const char TORA_TSTP_MST_SZSEHKHalt = 'e';
	///(港股通)持续交易
	const char TORA_TSTP_MST_SZSEHKContinous = 'f';
	///(港股通)Exchange Intervention
	const char TORA_TSTP_MST_SZSEHKExchangeIntervention = 'g';
	///(港股通)收盘集合竞价参考价定价
	const char TORA_TSTP_MST_SZSEHKCloseCallAuctionReferencePrice = 'h';
	///(港股通)收盘集合竞价输入买卖盘
	const char TORA_TSTP_MST_SZSEHKCloseCallAuctionInput = 'i';
	///(港股通)收盘集合竞价不可取消
	const char TORA_TSTP_MST_SZSEHKCloseCallAuctionCannotCancel = 'j';
	///(港股通)收盘集合竞价对盘
	const char TORA_TSTP_MST_SZSEHKCloseCallAuctionMatch = 'k';
	///(港股通)收盘集合竞价随机收市
	const char TORA_TSTP_MST_SZSEHKCloseCallAuctionRandomClosed = 'l';
	///(港股通)取消买卖盘
	const char TORA_TSTP_MST_SZSEHKCancel = 'm';
	///(港股通)收市
	const char TORA_TSTP_MST_SZSEHKClosed = 'n';
	///(港股通)全日收市
	const char TORA_TSTP_MST_SZSEHKWholeClosed = 'o';
	typedef char TTORATstpMarketStatusType;
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpTimeStampType是一个时间戳类型
	/////////////////////////////////////////////////////////////////////////
	typedef int TTORATstpTimeStampType;
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpFensVerType是一个版本号类型
	/////////////////////////////////////////////////////////////////////////
	typedef short TTORATstpFensVerType;
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpFensEnvIDType是一个环境编号类型
	/////////////////////////////////////////////////////////////////////////
	typedef char TTORATstpFensEnvIDType[13];
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpFensNodeIDType是一个节点编号类型
	/////////////////////////////////////////////////////////////////////////
	typedef char TTORATstpFensNodeIDType[11];
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpFensUserIDType是一个用户代码类型
	/////////////////////////////////////////////////////////////////////////
	typedef char TTORATstpFensUserIDType[16];
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpClientInfoType是一个终端信息类型
	/////////////////////////////////////////////////////////////////////////
	typedef char TTORATstpClientInfoType[65];
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpPortType是一个端口号类型
	/////////////////////////////////////////////////////////////////////////
	typedef int TTORATstpPortType;
	
	/////////////////////////////////////////////////////////////////////////
	/// TTORATstpMDSubModeType是一个行情订阅模式类型
	/////////////////////////////////////////////////////////////////////////
	///TCP连接模式
	const char TORA_TSTP_MST_TCP = '0';
	///UDP单播模式
	const char TORA_TSTP_MST_UDP = '1';
	///UDP组播模式
	const char TORA_TSTP_MST_MCAST = '2';
	typedef char TTORATstpMDSubModeType;
	
}
#endif // TORATSTPUSERAPIDATATYPE_H__